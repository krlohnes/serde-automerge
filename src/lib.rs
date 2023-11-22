#![doc = include_str!("../README.md")]

pub mod de;
pub mod ser;

pub use automerge::*;
pub use de::Deserializer;
pub use ser::Serializer;

#[derive(Debug, thiserror::Error)]
pub enum AutomergeSerdeError {
    #[error(transparent)]
    Serialize(#[from] ser::Error),
    #[error(transparent)]
    Deserialize(#[from] de::Error),
    #[error(transparent)]
    Automerge(#[from] AutomergeError),
}

pub trait AutomergeSetExtension {
    fn set_value<S: serde::Serialize, P: Into<Prop>>(
        &mut self,
        obj: ObjId,
        prop: P,
        value: S,
    ) -> Result<ObjId, AutomergeSerdeError>;
}
pub trait AutomergeGetExtension {
    fn get_value<'de, S: serde::Deserialize<'de>, P: Into<Prop>>(
        &self,
        obj: ObjId,
        prop: P,
    ) -> Result<Option<S>, AutomergeSerdeError>;
}

impl<'a> AutomergeSetExtension for transaction::Transaction<'a> {
    fn set_value<S: serde::Serialize, P: Into<Prop>>(
        &mut self,
        obj: ObjId,
        prop: P,
        value: S,
    ) -> Result<ObjId, AutomergeSerdeError> {
        value
            .serialize(Serializer::new(self, obj, prop))
            .map(|(_, id)| id)
            .map_err(Into::into)
    }
}

impl AutomergeSetExtension for AutoCommit {
    fn set_value<S: serde::Serialize, P: Into<Prop>>(
        &mut self,
        obj: ObjId,
        prop: P,
        value: S,
    ) -> Result<ObjId, AutomergeSerdeError> {
        value
            .serialize(Serializer::new(self, obj, prop))
            .map(|(_, id)| id)
            .map_err(Into::into)
    }
}

impl AutomergeSetExtension for Automerge {
    fn set_value<S: serde::Serialize, P: Into<Prop>>(
        &mut self,
        obj: ObjId,
        prop: P,
        value: S,
    ) -> Result<ObjId, AutomergeSerdeError> {
        let mut transaction = self.transaction();
        let id = transaction.set_value(obj, prop, value)?;
        transaction.commit();
        Ok(id)
    }
}

impl AutomergeGetExtension for Automerge {
    fn get_value<'de, S: serde::Deserialize<'de>, P: Into<Prop>>(
        &self,
        obj: ObjId,
        prop: P,
    ) -> Result<Option<S>, AutomergeSerdeError> {
        self.get(obj, prop)?
            .map(|(v, id)| Deserializer::new_found(self, v, id))
            .map(|d| S::deserialize(d))
            .transpose()
            .map_err(Into::into)
    }
}

pub trait AutomergeExtension: AutomergeSetExtension + AutomergeGetExtension {}
impl<T: AutomergeSetExtension + AutomergeGetExtension> AutomergeExtension for T {}
