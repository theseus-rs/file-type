use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109334805: FileFormat = FileFormat {
    id: 109_334_805,
    source_type: SourceType::Wikidata,
    name: "Advanced Comic Book Format",
    extensions: &["acbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
