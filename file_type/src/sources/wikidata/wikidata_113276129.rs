use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113276129: FileFormat = FileFormat {
    id: 113_276_129,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Photo Pages",
    extensions: &["pho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
