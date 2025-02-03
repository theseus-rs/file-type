use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125925165: FileFormat = FileFormat {
    id: 125_925_165,
    source_type: SourceType::Wikidata,
    name: "Papyrus Base Formula file",
    extensions: &["pbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
