use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62446408: FileFormat = FileFormat {
    id: 62_446_408,
    source_type: SourceType::Wikidata,
    name: "OWL Manchester Syntax",
    extensions: &["omn"],
    media_types: &["text/owl-manchester"],
    internal_signatures: &[],
    related_formats: &[],
};
