use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48551893: FileFormat = FileFormat {
    id: 48_551_893,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 5",
    extensions: &["ws", "ws5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
