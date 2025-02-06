use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124429367: FileFormat = FileFormat {
    id: 124_429_367,
    source_type: SourceType::Wikidata,
    name: "Pyramix Media File",
    extensions: &["pmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
