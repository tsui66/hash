import { parse } from "url";

// A URL router for the server.
export class Router {
  constructor() {
    this.routes = [];
  }

  add(method, url, handler) {
    this.routes.push({ method, url, handler });
  }

  // : (union<string, RegExp, Array>, string) → union<Array, null>
  // Check whether a route pattern matches a given URL path.
  match(pattern, path) {
    if (typeof pattern === "string") {
      if (pattern === path) return [];
    } else if (pattern instanceof RegExp) {
      const match = pattern.exec(path);
      return match && match.slice(1);
    } else {
      const parts = path.slice(1).split("/");
      if (parts.length && !parts[parts.length - 1]) parts.pop();
      if (parts.length !== pattern.length) return null;
      const result = [];
      for (let i = 0; i < parts.length; i++) {
        const pat = pattern[i];
        if (pat) {
          if (pat !== parts[i]) return null;
        } else {
          result.push(parts[i]);
        }
      }
      return result;
    }
  }

  // Resolve a request, letting the matching route write a response.
  resolve(request, response) {
    const parsed = parse(request.url, true);
    const path = parsed.pathname;
    request.query = parsed.query;

    return this.routes.some((route) => {
      const isOptions = request.method === "OPTIONS";
      const match =
        (isOptions || route.method === request.method) &&
        this.match(route.url, path);
      if (!match) return false;

      // Set CORS headers
      response.setHeader("Access-Control-Allow-Origin", "*");
      response.setHeader("Access-Control-Request-Method", "*");
      // @todo send correct allowed methods
      response.setHeader("Access-Control-Allow-Methods", "OPTIONS, GET, POST");
      response.setHeader("Access-Control-Allow-Headers", "*");
      if (isOptions) {
        response.writeHead(200);
        response.end();
        return true;
      }

      const urlParts = match.map(decodeURIComponent);
      route.handler(request, response, ...urlParts);
      return true;
    });
  }
}
