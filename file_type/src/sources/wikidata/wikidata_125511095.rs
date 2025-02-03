use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125511095: FileFormat = FileFormat {
    id: 125_511_095,
    source_type: SourceType::Wikidata,
    name: "Zoner Photo Studio file",
    extensions: &["zps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
