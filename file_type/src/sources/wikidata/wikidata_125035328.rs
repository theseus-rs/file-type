use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125035328: FileFormat = FileFormat {
    id: 125_035_328,
    source_type: SourceType::Wikidata,
    name: "TinkerPlots document",
    extensions: &["tp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
