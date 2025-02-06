use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51954383: FileFormat = FileFormat {
    id: 51_954_383,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 5.5",
    extensions: &["ws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
