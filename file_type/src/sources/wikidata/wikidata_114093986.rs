use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114093986: FileFormat = FileFormat {
    id: 114_093_986,
    source_type: SourceType::Wikidata,
    name: "Sony SML File",
    extensions: &["sml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
