use crate::format::{RelatedFormat, Signature};
use core::cmp::Ordering;

/// The source of the file format
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SourceType {
    #[default]
    Default,
    Httpd,
    Iana,
    Linguist,
    Pronom,
    Wikidata,
}

impl SourceType {
    /// Get the priority of the source type
    #[must_use]
    pub fn priority(&self) -> usize {
        match self {
            SourceType::Default => 0,
            SourceType::Httpd => 4,
            SourceType::Iana => 5,
            SourceType::Linguist => 3,
            SourceType::Pronom => 2,
            SourceType::Wikidata => 1,
        }
    }
}

impl PartialOrd for SourceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority().cmp(&other.priority()))
    }
}

impl Ord for SourceType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

/// A file format and its associated information
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileFormat {
    pub id: usize,
    pub source_type: SourceType,
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub media_types: &'static [&'static str],
    pub signatures: &'static [Signature],
    pub related_formats: &'static [RelatedFormat],
}

impl FileFormat {
    /// Check if this file format is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        let matched = self
            .signatures
            .iter()
            .any(|signature| signature.is_match(bytes));
        matched
    }
}

impl Ord for FileFormat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.source_type.cmp(&other.source_type) {
            Ordering::Equal => self.id.cmp(&other.id),
            ordering => ordering,
        }
    }
}

impl PartialOrd for FileFormat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(SourceType::Default.priority(), 0);
        assert_eq!(SourceType::Httpd.priority(), 4);
        assert_eq!(SourceType::Iana.priority(), 5);
        assert_eq!(SourceType::Linguist.priority(), 3);
        assert_eq!(SourceType::Pronom.priority(), 2);
        assert_eq!(SourceType::Wikidata.priority(), 1);
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(
            SourceType::Wikidata.partial_cmp(&SourceType::Wikidata),
            Some(Ordering::Equal)
        );
        assert_eq!(
            SourceType::Wikidata.partial_cmp(&SourceType::Default),
            Some(Ordering::Greater)
        );
        assert_eq!(
            SourceType::Default.partial_cmp(&SourceType::Wikidata),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_cmp() {
        assert_eq!(
            SourceType::Wikidata.cmp(&SourceType::Wikidata),
            Ordering::Equal
        );
        assert_eq!(
            SourceType::Wikidata.cmp(&SourceType::Default),
            Ordering::Greater
        );
        assert_eq!(
            SourceType::Default.cmp(&SourceType::Wikidata),
            Ordering::Less
        );
    }
}
