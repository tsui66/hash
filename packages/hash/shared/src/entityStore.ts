import { Draft, produce } from "immer";
import { v4 as uuid } from "uuid";
import { AnyEntity, BlockEntity } from "./entity";
import { DistributiveOmit } from "./util";

// @todo should AnyEntity include BlockEntity, and should this just be AnyEntity
export type EntityStoreType = BlockEntity | AnyEntity;

type PropertiesType<Properties extends {}> = Properties extends {
  entity: EntityStoreType;
}
  ? DistributiveOmit<Properties, "entity"> & {
      entity: DraftEntity<Properties["entity"]>;
    }
  : Properties;

export type DraftEntity<Type extends EntityStoreType = EntityStoreType> = {
  entityId: Type["entityId"] | null;
  draftId: string;
} & (Type extends { properties: any }
  ? { properties: PropertiesType<Type["properties"]> }
  : {});

export type EntityStore = {
  saved: Record<string, EntityStoreType>;
  draft: Record<string, DraftEntity>;
};

/**
 * @todo should be more robust
 */
export const isEntity = (value: unknown): value is EntityStoreType =>
  typeof value === "object" && value !== null && "entityId" in value;

// @todo does this need to be more robust?
export const isBlockEntity = (entity: unknown): entity is BlockEntity =>
  isEntity(entity) &&
  "properties" in entity &&
  entity.properties &&
  "entity" in entity.properties &&
  isEntity(entity.properties.entity);

export const isDraftBlockEntity = (
  entity: unknown,
): entity is DraftEntity<BlockEntity> =>
  isBlockEntity(entity) && "draftId" in entity;

/**
 * @todo we could store a map of entity id <-> draft id to make this easier
 */
export const draftEntityForEntityId = (
  draft: EntityStore["draft"],
  entityId: string,
) => Object.values(draft).find((entity) => entity.entityId === entityId);

/**
 * @todo work on performance
 */
export const walkValueForEntity = <T>(
  value: T,
  entityHandler: (entity: EntityStoreType) => EntityStoreType,
): T => {
  if (typeof value !== "object" || value === null) {
    return value;
  }

  let valueCopy = (
    Array.isArray(value) ? [...value] : { ...value }
  ) as typeof value;

  for (const [key, innerValue] of Object.entries(valueCopy)) {
    // @todo this is type safe, but TS doesn't know it
    // @ts-expect-error .................
    valueCopy[key] = walkValueForEntity(innerValue, entityHandler);
  }

  if (isEntity(valueCopy)) {
    // @todo this is type safe, but TS doesn't know it
    valueCopy = entityHandler(valueCopy) as any;
  }

  return valueCopy;
};

// /**
//  * @todo this only finds block entities and their immediate descendants
//  */
// const findEntitiesInValue = (value: unknown): EntityStoreType[] => {
//   let entities: EntityStoreType[] = [];
//
//   if (isBlockEntity(value)) {
//     entities = [...entities, value, value.properties.entity];
//   }
//
//   if (typeof value === "object" && value !== null) {
//     for (const property of Object.values(value)) {
//       entities = [...entities, ...findEntitiesInValue(property)];
//     }
//   }
//
//   return entities;
// };

function findEntities(contents: EntityStoreType[]) {
  const entities: EntityStoreType[] = [];

  walkValueForEntity(contents, (entity) => {
    if (isBlockEntity(entity)) {
      entities.push(entity, entity.properties.entity);
    }
    return entity;
  });

  return entities;
}

/**
 * @todo restore dealing with links
 * @todo this should be flat – so that we don't have to traverse links
 */
export const createEntityStore = (
  contents: EntityStoreType[],
  draftData: Record<string, DraftEntity>,
): EntityStore => {
  const saved: EntityStore["saved"] = {};
  const draft: EntityStore["draft"] = {};

  const draftToEntity: Record<string, string | null> = {};
  const entityToDraft: Record<string, string> = {};

  for (const row of Object.values(draftData)) {
    draftToEntity[row.draftId!] = row.entityId;
    if (row.entityId) {
      entityToDraft[row.entityId] = row.draftId;
    }
  }
  const entities = findEntities(contents);

  for (const entity of entities) {
    if (!entityToDraft[entity.entityId]) {
      entityToDraft[entity.entityId] = uuid();
    }
  }

  for (const entity of entities) {
    saved[entity.entityId] = entity;
    const draftId = entityToDraft[entity.entityId];

    /**
     * We current violate Immer's rules, as properties inside entities can be
     * other entities themselves, and we expect `entity.property.entity` to be
     * the same object as the other entity. We either need to change that, or
     * remove immer, or both.
     *
     * @todo address this
     * @see https://immerjs.github.io/immer/pitfalls#immer-only-supports-unidirectional-trees
     */
    draft[draftId] = produce<DraftEntity>(
      { ...entity, draftId },
      (draftEntity: Draft<DraftEntity>) => {
        if (draftData[draftId]) {
          Object.assign(draftEntity, draftData[draftId]);
        }
      },
    );

    draft[draftId] = produce<DraftEntity>(
      draft[draftId],
      (draftEntity: Draft<DraftEntity>) => {
        if (isBlockEntity(draftEntity)) {
          draftEntity.properties.entity.draftId =
            entityToDraft[draftEntity.properties.entity.entityId];
        }
      },
    );
  }

  return { saved, draft };
};
