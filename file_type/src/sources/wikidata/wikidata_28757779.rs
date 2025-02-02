use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757779: FileFormat = FileFormat {
    id: 28_757_779,
    source_type: SourceType::Wikidata,
    name: "GME",
    extensions: &["gme"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
