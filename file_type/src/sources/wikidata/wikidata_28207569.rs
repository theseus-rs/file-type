use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207569: FileFormat = FileFormat {
    id: 28_207_569,
    source_type: SourceType::Wikidata,
    name: "Zeiss BIVAS",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
