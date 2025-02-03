use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51954390: FileFormat = FileFormat {
    id: 51_954_390,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 6",
    extensions: &["ws", "ws6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
