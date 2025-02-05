use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2701652: FileFormat = FileFormat {
    id: 2_701_652,
    source_type: SourceType::Wikidata,
    name: "BSP",
    extensions: &["bsp"],
    media_types: &["model/vnd.valve.source.compiled-map"],
    signatures: &[],
    related_formats: &[],
};
