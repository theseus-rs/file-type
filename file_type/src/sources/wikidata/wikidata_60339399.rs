use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60339399: FileFormat = FileFormat {
    id: 60_339_399,
    source_type: SourceType::Wikidata,
    name: "Open Project File",
    extensions: &["pod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
