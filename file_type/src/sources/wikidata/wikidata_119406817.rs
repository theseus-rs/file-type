use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119406817: FileFormat = FileFormat {
    id: 119_406_817,
    source_type: SourceType::Wikidata,
    name: "ACT! Database Pointer File",
    extensions: &["pad"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
