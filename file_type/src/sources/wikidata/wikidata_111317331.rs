use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317331: FileFormat = FileFormat {
    id: 111_317_331,
    source_type: SourceType::Wikidata,
    name: "Native Instruments Reaktor format",
    extensions: &["map"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
