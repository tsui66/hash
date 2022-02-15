import {
  Resolver,
  EntityType as GQLEntityType,
  JsonSchemaMeta,
} from "../../apiTypes.gen";
import { GraphQLContext } from "../../context";
import { EntityType, UnresolvedGQLEntityType } from "../../../model";
import {
  generateSchema$id,
  schema$idRef,
} from "../../../lib/schemas/jsonSchema";

const entityTypeChildrenResolver: Resolver<
  Promise<UnresolvedGQLEntityType[]>,
  GQLEntityType,
  GraphQLContext
> = async (params, _, { dataSources: { db } }) => {
  const { accountId, entityId: entityTypeId } = params;
  const schema$ID = generateSchema$id(accountId, entityTypeId);
  const schemaRef = schema$idRef(schema$ID);

  const entityTypes = await EntityType.getEntityTypeChildren(db, { schemaRef });

  return entityTypes.map((entityType) => entityType.toGQLEntityType());
};

const entityTypeParentsResolver: Resolver<
  Promise<UnresolvedGQLEntityType[]>,
  GQLEntityType,
  GraphQLContext
> = async (params, _, { dataSources }) => {
  const { entityId: entityTypeId } = params;

  const entityTypes = await EntityType.getEntityTypeParents(dataSources.db, {
    entityTypeId,
  });

  return entityTypes.map((ent) => ent.toGQLEntityType());
};

const entityTypeDestructuredSchemaResolver: Resolver<
  Promise<JsonSchemaMeta>,
  GQLEntityType,
  GraphQLContext
> = async (params, _, { dataSources: { db } }) => {
  const { entityId: entityTypeId } = params;

  const jsonSchemaMeta = await db.getEntityTypeJsonSchemaMeta({ entityTypeId });

  return jsonSchemaMeta;
};

export const entityTypeInheritance = {
  entityTypeChildren: entityTypeChildrenResolver,
  entityTypeParents: entityTypeParentsResolver,
  entityTypeDestructuredSchema: entityTypeDestructuredSchemaResolver,
};
