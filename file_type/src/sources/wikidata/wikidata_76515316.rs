use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76515316: FileFormat = FileFormat {
    id: 76_515_316,
    source_type: SourceType::Wikidata,
    name: "MapInfo Workspace",
    extensions: &["wor"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
