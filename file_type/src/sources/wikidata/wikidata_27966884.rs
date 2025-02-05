use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966884: FileFormat = FileFormat {
    id: 27_966_884,
    source_type: SourceType::Wikidata,
    name: "Direct Stream Digital Audio",
    extensions: &["dsf", "dsflib", "minidsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
