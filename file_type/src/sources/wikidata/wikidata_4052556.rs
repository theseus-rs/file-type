use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4052556: FileFormat = FileFormat {
    id: 4_052_556,
    source_type: SourceType::Wikidata,
    name: "Vector Quantized Animation",
    extensions: &["vqa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
