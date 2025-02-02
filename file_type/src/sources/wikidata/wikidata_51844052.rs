use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51844052: FileFormat = FileFormat {
    id: 51_844_052,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual Modeller Petal file (ASCII)",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
