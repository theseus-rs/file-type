use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113486947: FileFormat = FileFormat {
    id: 113_486_947,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 3.0",
    extensions: &["pr3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
