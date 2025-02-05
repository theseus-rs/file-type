use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485571: FileFormat = FileFormat {
    id: 117_485_571,
    source_type: SourceType::Wikidata,
    name: "Audacity Audio Block File",
    extensions: &["au"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
