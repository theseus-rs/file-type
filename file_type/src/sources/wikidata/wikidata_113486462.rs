use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113486462: FileFormat = FileFormat {
    id: 113_486_462,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 2.0",
    extensions: &["pr2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
