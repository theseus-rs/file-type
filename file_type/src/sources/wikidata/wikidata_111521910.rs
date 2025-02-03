use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111521910: FileFormat = FileFormat {
    id: 111_521_910,
    source_type: SourceType::Wikidata,
    name: "Packed-Ice True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
