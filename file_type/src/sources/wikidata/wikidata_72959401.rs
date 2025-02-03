use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72959401: FileFormat = FileFormat {
    id: 72_959_401,
    source_type: SourceType::Wikidata,
    name: "Panorama database",
    extensions: &["pan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
