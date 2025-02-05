use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4052556: FileFormat = FileFormat {
    id: 4_052_556,
    source_type: SourceType::Wikidata,
    name: "Vector Quantized Animation",
    extensions: &["vqa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
