use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114078864: FileFormat = FileFormat {
    id: 114_078_864,
    source_type: SourceType::Wikidata,
    name: "MacBinary II",
    extensions: &["bin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
