import { print } from "graphql/language/printer";
import { GraphQLClient, ClientError } from "graphql-request";

import {
  SendLoginCodeMutation,
  SendLoginCodeMutationVariables,
  CreateEntityMutation,
  CreateEntityMutationVariables,
  CreateOrgMutation,
  CreateOrgMutationVariables,
  CreatePageMutation,
  CreatePageMutationVariables,
  CreateUserMutation,
  CreateUserMutationVariables,
  InsertBlocksIntoPageMutation,
  InsertBlocksIntoPageMutationVariables,
  LoginWithLoginCodeMutationVariables,
  LoginWithLoginCodeMutation,
  GetEntityQueryVariables,
  GetEntityQuery,
  UpdateEntityMutationVariables,
  UpdateEntityMutation,
  GetPageQueryVariables,
  GetPageQuery,
  UpdatePageContentsMutation,
  UpdatePageContentsMutationVariables,
  CreateEntityTypeMutation,
  CreateEntityTypeMutationVariables,
  UpdateEntityTypeMutation,
  UpdateEntityTypeMutationVariables,
  CreateOrgEmailInvitationMutationVariables,
  CreateOrgEmailInvitationMutation,
  CreateUserWithOrgEmailInvitationMutationVariables,
  CreateUserWithOrgEmailInvitationMutation,
  GetOrgEmailInvitationQueryVariables,
  GetOrgEmailInvitationQuery,
  GetOrgInvitationLinkQueryVariables,
  GetOrgInvitationLinkQuery,
  JoinOrgMutationVariables,
  JoinOrgMutation,
} from "../graphql/apiTypes.gen";
import {
  createEntity,
  createEntityType,
  getUnknownEntity,
  updateEntity,
  updateEntityType,
} from "../graphql/queries/entity.queries";
import {
  createOrg,
  createOrgEmailInvitation,
  joinOrg,
  orgEmailInvitation,
  orgInvitationLink,
} from "../graphql/queries/org.queries";
import {
  createUser,
  createUserWithOrgEmailInvitation,
  loginWithLoginCode,
  sendLoginCode,
} from "../graphql/queries/user.queries";
import {
  createPage,
  insertBlocksIntoPage,
  getPage,
  updatePageContents,
} from "../graphql/queries/page.queries";

export class ApiClient {
  private client: GraphQLClient;

  constructor(url: string) {
    this.client = new GraphQLClient(url);
  }

  static getErrorCodesFromClientError = (clientError: ClientError) => {
    if (!clientError.response.errors) {
      throw new Error("No response errors found on client error");
    }
    return clientError.response.errors.map(
      (error: any) => error.extensions.code,
    );
  };

  setCookie = (cookie: string) => this.client.setHeader("Cookie", cookie);

  removeCookie = () => this.client.setHeader("Cookie", "");

  /** Sign-up related requests */

  createUser = async (vars: CreateUserMutationVariables) =>
    this.client
      .request<CreateUserMutation, CreateUserMutationVariables>(
        createUser,
        vars,
      )
      .then((res) => res.createUser);

  createUserWithOrgEmailInvitation = async (
    vars: CreateUserWithOrgEmailInvitationMutationVariables,
  ) =>
    this.client
      .request<
        CreateUserWithOrgEmailInvitationMutation,
        CreateUserWithOrgEmailInvitationMutationVariables
      >(createUserWithOrgEmailInvitation, vars)
      .then((res) => res.createUserWithOrgEmailInvitation);

  /** Log-in related requests */

  async sendLoginCode(vars: SendLoginCodeMutationVariables) {
    return (
      await this.client.request<
        SendLoginCodeMutation,
        SendLoginCodeMutationVariables
      >(sendLoginCode, vars)
    ).sendLoginCode;
  }

  async loginWithLoginCode(vars: LoginWithLoginCodeMutationVariables) {
    const { data, headers } = await this.client.rawRequest<
      LoginWithLoginCodeMutation,
      LoginWithLoginCodeMutationVariables
    >(print(loginWithLoginCode), vars);

    if (!data) {
      throw new Error("loginWithLoginCode mutation did not return data");
    }

    return { user: data.loginWithLoginCode, responseHeaders: headers };
  }

  /** Other requests */

  getUnknownEntity = async (vars: GetEntityQueryVariables) =>
    this.client
      .request<GetEntityQuery, GetEntityQueryVariables>(getUnknownEntity, vars)
      .then((res) => res.entity);

  updateEntity = async (vars: UpdateEntityMutationVariables) =>
    this.client
      .request<UpdateEntityMutation, UpdateEntityMutationVariables>(
        updateEntity,
        vars,
      )
      .then((res) => res.updateEntity);

  async createEntity(vars: CreateEntityMutationVariables) {
    return (
      await this.client.request<
        CreateEntityMutation,
        CreateEntityMutationVariables
      >(createEntity, vars)
    ).createEntity;
  }

  async createOrg(vars: CreateOrgMutationVariables) {
    return (
      await this.client.request<CreateOrgMutation, CreateOrgMutationVariables>(
        createOrg,
        vars,
      )
    ).createOrg;
  }

  async createOrgEmailInvitation(
    vars: CreateOrgEmailInvitationMutationVariables,
  ) {
    return (
      await this.client.request<
        CreateOrgEmailInvitationMutation,
        CreateOrgEmailInvitationMutationVariables
      >(createOrgEmailInvitation, vars)
    ).createOrgEmailInvitation;
  }

  async getOrgEmailInvitation(vars: GetOrgEmailInvitationQueryVariables) {
    return (
      await this.client.request<
        GetOrgEmailInvitationQuery,
        GetOrgEmailInvitationQueryVariables
      >(orgEmailInvitation, vars)
    ).getOrgEmailInvitation;
  }

  async getOrgInvitationLink(vars: GetOrgInvitationLinkQueryVariables) {
    return (
      await this.client.request<
        GetOrgInvitationLinkQuery,
        GetOrgInvitationLinkQueryVariables
      >(orgInvitationLink, vars)
    ).getOrgInvitationLink;
  }

  async joinOrg(vars: JoinOrgMutationVariables) {
    return (
      await this.client.request<JoinOrgMutation, JoinOrgMutationVariables>(
        joinOrg,
        vars,
      )
    ).joinOrg;
  }

  async createPage(vars: CreatePageMutationVariables) {
    return (
      await this.client.request<
        CreatePageMutation,
        CreatePageMutationVariables
      >(createPage, vars)
    ).createPage;
  }

  async createEntityType(vars: CreateEntityTypeMutationVariables) {
    return (
      await this.client.request<
        CreateEntityTypeMutation,
        CreateEntityTypeMutationVariables
      >(createEntityType, vars)
    ).createEntityType;
  }

  async updateEntityType(vars: UpdateEntityTypeMutationVariables) {
    return (
      await this.client.request<
        UpdateEntityTypeMutation,
        UpdateEntityTypeMutationVariables
      >(updateEntityType, vars)
    ).updateEntityType;
  }

  insertBlocksIntoPage = async (vars: InsertBlocksIntoPageMutationVariables) =>
    this.client
      .request<
        InsertBlocksIntoPageMutation,
        InsertBlocksIntoPageMutationVariables
      >(insertBlocksIntoPage, vars)
      .then((res) => res.insertBlocksIntoPage);

  getPage = async (vars: GetPageQueryVariables) =>
    this.client
      .request<GetPageQuery, GetPageQueryVariables>(getPage, vars)
      .then((res) => res.page);

  updatePageContents = async (vars: UpdatePageContentsMutationVariables) =>
    this.client
      .request<UpdatePageContentsMutation, UpdatePageContentsMutationVariables>(
        updatePageContents,
        vars,
      )
      .then((res) => res.updatePageContents);
}
