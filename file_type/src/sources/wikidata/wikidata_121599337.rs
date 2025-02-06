use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121599337: FileFormat = FileFormat {
    id: 121_599_337,
    source_type: SourceType::Wikidata,
    name: "Hallmark Card Studio Project File",
    extensions: &["hmk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
