use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287127: FileFormat = FileFormat {
    id: 117_287_127,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot 1.0, 2.0 Worksheet",
    extensions: &["spw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
