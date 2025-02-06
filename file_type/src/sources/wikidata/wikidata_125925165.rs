use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125925165: FileFormat = FileFormat {
    id: 125_925_165,
    source_type: SourceType::Wikidata,
    name: "Papyrus Base Formula file",
    extensions: &["pbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
