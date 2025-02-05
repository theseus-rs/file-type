use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125172737: FileFormat = FileFormat {
    id: 125_172_737,
    source_type: SourceType::Wikidata,
    name: "MyNotex file",
    extensions: &["mnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
