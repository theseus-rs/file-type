use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51954568: FileFormat = FileFormat {
    id: 51_954_568,
    source_type: SourceType::Wikidata,
    name: "WordStar for Windows Document, version 2",
    extensions: &["ws", "wsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
