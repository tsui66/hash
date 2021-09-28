import { ApolloError } from "apollo-server-express";

import { QueryOrgInvitationLinkArgs, Resolver } from "../../apiTypes.gen";
import { LoggedInGraphQLContext } from "../../context";
import { Org, EntityTypeWithoutTypeFields } from "../../../model";

export const orgInvitationLink: Resolver<
  Promise<EntityTypeWithoutTypeFields>,
  {},
  LoggedInGraphQLContext,
  QueryOrgInvitationLinkArgs
> = async (_, { orgEntityId, invitationLinkToken }, { dataSources }) =>
  dataSources.db.transaction(async (client) => {
    const org = await Org.getOrgById(client)({ entityId: orgEntityId });

    if (!org) {
      const msg = `Org with entityId ${orgEntityId} not found in datastore`;
      throw new ApolloError(msg, "ORG_NOT_FOUND");
    }

    const invitation = await org.getInvitationWithToken(client)(
      invitationLinkToken
    );

    const errorMsgPrefix = `The invitation with token ${invitationLinkToken} associated with org with entityId ${orgEntityId}`;

    if (!invitation) {
      const msg = `${errorMsgPrefix} could not be found in the datastore.`;
      throw new ApolloError(msg, "NOT_FOUND");
    }

    if (invitation.hasBeenRevoked()) {
      const msg = `${errorMsgPrefix} has been revoked.`;
      throw new ApolloError(msg, "REVOKED");
    }

    return invitation.toGQLUnknownEntity();
  });
