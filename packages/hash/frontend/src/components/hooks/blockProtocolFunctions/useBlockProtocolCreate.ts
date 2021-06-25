import { useMutation } from "@apollo/client";

import { BlockProtocolCreateFn } from "../../../types/blockProtocol";
import { createEntity } from "../../../graphql/queries/entity.queries";
import { useCallback } from "react";
import {
  CreateEntityMutation,
  CreateEntityMutationVariables,
} from "../../../graphql/autoGeneratedTypes";

export const useBlockProtocolCreate = (): {
  create: BlockProtocolCreateFn;
  createLoading: boolean;
  createError: any;
} => {
  const [createFn, { loading: createLoading, error: createError }] =
    useMutation<CreateEntityMutation, CreateEntityMutationVariables>(
      createEntity
    );

  const create: BlockProtocolCreateFn = useCallback((actions) => {
    for (const action of actions) {
      createFn({
        variables: {
          properties: action.data,
          type: action.entityType
        },
      });
    }
  }, []);

  return {
    create,
    createLoading,
    createError,
  };
};
