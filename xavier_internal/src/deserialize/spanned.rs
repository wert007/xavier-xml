use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::deserialize::macro_trait::XmlDeserializable;

pub struct Spanned<T> {
    pub span: quick_xml::reader::Span,
    pub inner: T,
}

// unsafe impl<T: Copy> Copy for Spanned<T> {}

impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Self {
            span: self.span.clone(),
            inner: self.inner.clone(),
        }
    }
}
impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spanned")
            .field("span", &self.span)
            .field("inner", &self.inner)
            .finish()
    }
}
impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}
impl<T: Eq> Eq for Spanned<T> {}
impl<T: Ord> Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}
impl<T: XmlDeserializable> XmlDeserializable for Spanned<T> {
    fn from_xml(
        reader: &mut quick_xml::Reader<&[u8]>,
        event: Option<&quick_xml::events::BytesStart>,
        tag_name: Option<&str>,
    ) -> Result<Option<Self>, super::error::PError>
    where
        Self: Sized,
    {
        let start = reader.buffer_position();
        let inner = T::from_xml(reader, event, tag_name)?;
        let end = reader.buffer_position();
        Ok(match inner {
            Some(inner) => Some(Spanned {
                inner,
                span: start..end,
            }),
            None => None,
        })
    }
}

impl<T> Spanned<T> {
    pub fn span(&self) -> quick_xml::reader::Span {
        self.span.clone()
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
