import { FormEvent, useState, VoidFunctionComponent } from "react";

import { useRouter } from "next/router";

import { useMutation } from "@apollo/client";
import { tw } from "twind";
import styles from "../../index.module.scss";
import { PageSidebar } from "../../../components/layout/PageSidebar/PageSidebar";
import { createEntityTypeMutation } from "../../../graphql/queries/entityType.queries";
import {
  CreateEntityTypeMutation,
  CreateEntityTypeMutationVariables,
} from "../../../graphql/apiTypes.gen";
import { TextInput } from "../../../components/forms/TextInput";
import { Button } from "../../../components/forms/Button";

export const NewEntityType: VoidFunctionComponent = () => {
  const router = useRouter();
  const { query } = router;
  const accountId = query.accountId as string;

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  const [createEntityType] = useMutation<
    CreateEntityTypeMutation,
    CreateEntityTypeMutationVariables
  >(createEntityTypeMutation, {
    onCompleted: ({ createEntityType: entityType }) =>
      router.push(`/${entityType.accountId}/types/${entityType.entityId}`),
  });

  const submit = (event: FormEvent) => {
    event.preventDefault();
    createEntityType({ variables: { description, name, accountId } }).catch(
      (err) => console.error("Could not create EntityType: ", err)
    );
  };

  return (
    <div className={styles.MainWrapper}>
      <PageSidebar />
      <main className={styles.MainContent}>
        <header className={tw`mb-12`}>
          <h1>
            Create new <strong>entity type</strong>
          </h1>
          <p>
            Entity types (sometimes called ‘schemas’) are used to define
            entities. Use them to add new custom entities to your graph.
          </p>
        </header>
        <section>
          <form onSubmit={submit}>
            <div className={tw`max-w-2xl lg:(flex justify-between) mb-8`}>
              <TextInput
                className={tw`w-full mb-6 lg:(mb-0 w-72)`}
                disallowRegExp={/\W/g}
                label="Name"
                onChangeText={setName}
                value={name}
              />
              <TextInput
                className={tw`w-full lg:w-72`}
                label="Description"
                onChangeText={setDescription}
                value={description}
              />
            </div>
            <div>
              <Button type="submit">Create Entity Type</Button>
            </div>
          </form>
        </section>
      </main>
    </div>
  );
};

export default NewEntityType;
