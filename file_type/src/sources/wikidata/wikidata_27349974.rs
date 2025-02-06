use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349974: FileFormat = FileFormat {
    id: 27_349_974,
    source_type: SourceType::Wikidata,
    name: "TOPSAR Incidence Angle Map",
    extensions: &["incgr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
