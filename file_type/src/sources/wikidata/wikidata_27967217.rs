use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967217: FileFormat = FileFormat {
    id: 27_967_217,
    source_type: SourceType::Wikidata,
    name: "Scream Tracker Music Interface Kit module",
    extensions: &["stx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
