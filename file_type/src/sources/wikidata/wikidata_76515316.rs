use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76515316: FileFormat = FileFormat {
    id: 76_515_316,
    source_type: SourceType::Wikidata,
    name: "MapInfo Workspace",
    extensions: &["wor"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
