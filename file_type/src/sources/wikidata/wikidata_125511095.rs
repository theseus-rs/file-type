use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125511095: FileFormat = FileFormat {
    id: 125_511_095,
    source_type: SourceType::Wikidata,
    name: "Zoner Photo Studio file",
    extensions: &["zps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
