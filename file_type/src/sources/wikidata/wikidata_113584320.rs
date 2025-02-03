use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113584320: FileFormat = FileFormat {
    id: 113_584_320,
    source_type: SourceType::Wikidata,
    name: "Viscosity file",
    extensions: &["vsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
