use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967217: FileFormat = FileFormat {
    id: 27_967_217,
    source_type: SourceType::Wikidata,
    name: "Scream Tracker Music Interface Kit module",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
