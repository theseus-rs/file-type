use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117318924: FileFormat = FileFormat {
    id: 117_318_924,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Graphic 2.0",
    extensions: &["wp2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
