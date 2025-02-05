use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119257000: FileFormat = FileFormat {
    id: 119_257_000,
    source_type: SourceType::Wikidata,
    name: "PayCycle Import Data",
    extensions: &["pcif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
