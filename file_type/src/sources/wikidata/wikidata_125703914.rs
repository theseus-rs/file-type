use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125703914: FileFormat = FileFormat {
    id: 125_703_914,
    source_type: SourceType::Wikidata,
    name: "StarWriter Graphics Format",
    extensions: &["sgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
