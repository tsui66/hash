import { ApolloError } from "apollo-server-express";

import {
  QueryEntityArgs,
  Resolver,
  Visibility,
} from "../../autoGeneratedTypes";
import { DbUnknownEntity } from "../../../types/dbTypes";
import { GraphQLContext } from "../../context";

export const entity: Resolver<
  Promise<DbUnknownEntity>,
  {},
  GraphQLContext,
  QueryEntityArgs
> = async (_, { namespaceId, id }, { dataSources }) => {
  const dbEntity = await dataSources.db.getEntity({ namespaceId, id });
  if (!dbEntity) {
    throw new ApolloError(
      `Entidy id ${id} not found in namespace ${namespaceId}`
    );
  }

  const entity: DbUnknownEntity = {
    ...dbEntity,
    visibility: Visibility.Public, // TODO: should be a param?
  };

  return entity;
};
