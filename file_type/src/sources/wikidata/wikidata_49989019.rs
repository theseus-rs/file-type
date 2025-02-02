use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49989019: FileFormat = FileFormat {
    id: 49_989_019,
    source_type: SourceType::Wikidata,
    name: "Macromedia (Adobe) Director Compressed Resource file",
    extensions: &["dcr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
