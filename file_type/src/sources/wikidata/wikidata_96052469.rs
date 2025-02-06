use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96052469: FileFormat = FileFormat {
    id: 96_052_469,
    source_type: SourceType::Wikidata,
    name: "Mathematica Graphics Format",
    extensions: &["mgf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
