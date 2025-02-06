use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28212272: FileFormat = FileFormat {
    id: 28_212_272,
    source_type: SourceType::Wikidata,
    name: "Noweb",
    extensions: &["nw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
