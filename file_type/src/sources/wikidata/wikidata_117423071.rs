use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117423071: FileFormat = FileFormat {
    id: 117_423_071,
    source_type: SourceType::Wikidata,
    name: "Stimulus file",
    extensions: &["stm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
