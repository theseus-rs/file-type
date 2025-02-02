use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96052469: FileFormat = FileFormat {
    id: 96_052_469,
    source_type: SourceType::Wikidata,
    name: "Mathematica Graphics Format",
    extensions: &["mgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
