use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113521033: FileFormat = FileFormat {
    id: 113_521_033,
    source_type: SourceType::Wikidata,
    name: "BIM Metadata File",
    extensions: &["bim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
