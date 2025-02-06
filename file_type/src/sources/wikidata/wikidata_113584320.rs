use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113584320: FileFormat = FileFormat {
    id: 113_584_320,
    source_type: SourceType::Wikidata,
    name: "Viscosity file",
    extensions: &["vsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
