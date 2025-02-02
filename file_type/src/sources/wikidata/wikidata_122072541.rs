use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122072541: FileFormat = FileFormat {
    id: 122_072_541,
    source_type: SourceType::Wikidata,
    name: "Rhapsody File",
    extensions: &["rhp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
