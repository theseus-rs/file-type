use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72959401: FileFormat = FileFormat {
    id: 72_959_401,
    source_type: SourceType::Wikidata,
    name: "Panorama database",
    extensions: &["pan"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
