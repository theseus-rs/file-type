use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121599354: FileFormat = FileFormat {
    id: 121_599_354,
    source_type: SourceType::Wikidata,
    name: "Hallmark Card Studio format",
    extensions: &["hcs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
