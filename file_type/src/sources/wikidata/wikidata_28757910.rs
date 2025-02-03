use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757910: FileFormat = FileFormat {
    id: 28_757_910,
    source_type: SourceType::Wikidata,
    name: "Google Document",
    extensions: &["gdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
