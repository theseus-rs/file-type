use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52230534: FileFormat = FileFormat {
    id: 52_230_534,
    source_type: SourceType::Wikidata,
    name: "Polynomial Texture Map",
    extensions: &["ptm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
