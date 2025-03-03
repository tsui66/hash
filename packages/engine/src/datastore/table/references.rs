use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
};

use rayon::iter::ParallelIterator;

use super::{super::prelude::*, pool::message::MessageReader};
use crate::datastore::{table::pool::proxy::PoolReadProxy, UUID_V4_LEN};

#[derive(Clone, Debug)]
pub struct AgentMessageReference {
    pub batch_index: usize,
    pub agent_index: usize,
    pub message_index: usize,
}

impl AgentMessageReference {
    pub fn new(
        batch_index: usize,
        agent_index: usize,
        message_index: usize,
    ) -> AgentMessageReference {
        AgentMessageReference {
            batch_index,
            agent_index,
            message_index,
        }
    }
}

/// A mapping from recipient to message reference.
/// Used in combination with `MessageReader`.
pub struct MessageMap {
    inner: HashMap<String, Vec<AgentMessageReference>>,
}

impl MessageMap {
    pub fn new(pool: &PoolReadProxy<MessageBatch>) -> Result<MessageMap> {
        let iter = pool.recipient_iter_all();
        let inner = iter
            .fold(
                HashMap::<String, Vec<AgentMessageReference>>::new,
                |mut acc, (recipients, message_ref)| {
                    recipients.iter().for_each(|recipient| {
                        // TODO: OS - (decide) currently if message has duplicate recipients then
                        // agents can get duplicate messages (filtering is expensive)
                        if let Some(entry) = acc.get_mut(*recipient) {
                            entry.push(message_ref.clone())
                        } else {
                            acc.insert(recipient.to_string(), vec![message_ref.clone()]);
                        }
                    });
                    acc
                },
            )
            .reduce(HashMap::new, |mut a, b| {
                b.into_iter().for_each(|(name, mut value)| {
                    match a.entry(name) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().append(&mut value);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(value);
                        }
                    };
                });
                a
            });

        Ok(MessageMap { inner })
    }

    pub fn get_msg_refs(&self, recipient: &str) -> &[AgentMessageReference] {
        self.inner.get(recipient).map(Deref::deref).unwrap_or(&[])
    }

    pub fn get_types<'a: 'b, 'b>(
        &'b self,
        recipient: &str,
        reader: &'a MessageReader<'a>,
    ) -> Result<impl ParallelIterator<Item = &'a str> + 'b> {
        let references = self.inner.get(recipient).map(Deref::deref).unwrap_or(&[]);
        let types = reader.type_iter(references);
        Ok(types)
    }

    pub fn get_datas<'a: 'b, 'b>(
        &'b self,
        recipient: &str,
        reader: &'a MessageReader<'a>,
    ) -> Result<impl ParallelIterator<Item = &'a str> + 'b> {
        let references = self.inner.get(recipient).map(Deref::deref).unwrap_or(&[]);
        let datas = reader.data_iter(references);
        Ok(datas)
    }

    pub fn get_froms<'a: 'b, 'b>(
        &'b self,
        recipient: &str,
        reader: &'a MessageReader<'a>,
    ) -> Result<impl ParallelIterator<Item = &'a [u8; UUID_V4_LEN]> + 'b> {
        let references = self.inner.get(recipient).map(Deref::deref).unwrap_or(&[]);
        let froms = reader.from_iter(references);
        Ok(froms)
    }
}
