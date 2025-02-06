use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287222: FileFormat = FileFormat {
    id: 117_287_222,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot DOS Worksheet",
    extensions: &["spg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
