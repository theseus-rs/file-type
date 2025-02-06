use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73019618: FileFormat = FileFormat {
    id: 73_019_618,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 7.0",
    extensions: &["ws7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
