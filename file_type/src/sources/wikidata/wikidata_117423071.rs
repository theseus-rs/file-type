use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117423071: FileFormat = FileFormat {
    id: 117_423_071,
    source_type: SourceType::Wikidata,
    name: "Stimulus file",
    extensions: &["stm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
