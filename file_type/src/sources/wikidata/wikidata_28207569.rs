use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207569: FileFormat = FileFormat {
    id: 28_207_569,
    source_type: SourceType::Wikidata,
    name: "Zeiss BIVAS",
    extensions: &["dta"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
