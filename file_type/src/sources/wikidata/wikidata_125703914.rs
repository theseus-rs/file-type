use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125703914: FileFormat = FileFormat {
    id: 125_703_914,
    source_type: SourceType::Wikidata,
    name: "StarWriter Graphics Format",
    extensions: &["sgf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
