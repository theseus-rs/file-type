use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111521910: FileFormat = FileFormat {
    id: 111_521_910,
    source_type: SourceType::Wikidata,
    name: "Packed-Ice True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
