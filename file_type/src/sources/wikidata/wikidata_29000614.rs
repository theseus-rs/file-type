use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000614: FileFormat = FileFormat {
    id: 29_000_614,
    source_type: SourceType::Wikidata,
    name: "Resource File",
    extensions: &["res"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
