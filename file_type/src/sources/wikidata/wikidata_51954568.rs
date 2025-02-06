use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51954568: FileFormat = FileFormat {
    id: 51_954_568,
    source_type: SourceType::Wikidata,
    name: "WordStar for Windows Document, version 2",
    extensions: &["ws", "wsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
