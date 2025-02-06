use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113276129: FileFormat = FileFormat {
    id: 113_276_129,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Photo Pages",
    extensions: &["pho"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
