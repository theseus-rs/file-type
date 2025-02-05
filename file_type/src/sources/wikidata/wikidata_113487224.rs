use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113487224: FileFormat = FileFormat {
    id: 113_487_224,
    source_type: SourceType::Wikidata,
    name: "Persuasion Player File 3",
    extensions: &["ppf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
