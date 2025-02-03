use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117537130: FileFormat = FileFormat {
    id: 117_537_130,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Presentations 2",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
