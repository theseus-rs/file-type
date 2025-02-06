use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771316: FileFormat = FileFormat {
    id: 28_771_316,
    source_type: SourceType::Wikidata,
    name: "Micrografx Draw",
    extensions: &["drw"],
    media_types: &["application/x-mgx-designer"],
    signatures: &[],
    related_formats: &[],
};
