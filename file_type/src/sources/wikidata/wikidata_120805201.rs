use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120805201: FileFormat = FileFormat {
    id: 120_805_201,
    source_type: SourceType::Wikidata,
    name: "OCP Art Studio Screen File",
    extensions: &["SCR"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
