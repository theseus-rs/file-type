use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123973: FileFormat = FileFormat {
    id: 67_123_973,
    source_type: SourceType::Wikidata,
    name: "Print Artist certificate file format",
    extensions: &["cer"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
