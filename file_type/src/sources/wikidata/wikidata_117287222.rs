use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287222: FileFormat = FileFormat {
    id: 117_287_222,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot DOS Worksheet",
    extensions: &["spg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
