use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48551893: FileFormat = FileFormat {
    id: 48_551_893,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 5",
    extensions: &["ws", "ws5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
