use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120805201: FileFormat = FileFormat {
    id: 120_805_201,
    source_type: SourceType::Wikidata,
    name: "OCP Art Studio Screen File",
    extensions: &["SCR"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
