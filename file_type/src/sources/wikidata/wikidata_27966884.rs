use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966884: FileFormat = FileFormat {
    id: 27_966_884,
    source_type: SourceType::Wikidata,
    name: "Direct Stream Digital Audio",
    extensions: &["dsf", "dsflib", "minidsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
