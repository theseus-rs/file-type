use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118583205: FileFormat = FileFormat {
    id: 118_583_205,
    source_type: SourceType::Wikidata,
    name: "Project5 Project",
    extensions: &["p5p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
