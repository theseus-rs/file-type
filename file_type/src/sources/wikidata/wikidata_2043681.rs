use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2043681: FileFormat = FileFormat {
    id: 2_043_681,
    source_type: SourceType::Wikidata,
    name: "PAK",
    extensions: &["pak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
