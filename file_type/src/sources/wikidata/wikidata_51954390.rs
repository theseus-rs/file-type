use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51954390: FileFormat = FileFormat {
    id: 51_954_390,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 6",
    extensions: &["ws", "ws6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
