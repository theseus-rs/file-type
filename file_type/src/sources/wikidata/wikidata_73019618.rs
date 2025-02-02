use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73019618: FileFormat = FileFormat {
    id: 73_019_618,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 7.0",
    extensions: &["ws7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
