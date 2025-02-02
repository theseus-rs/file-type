use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121599337: FileFormat = FileFormat {
    id: 121_599_337,
    source_type: SourceType::Wikidata,
    name: "Hallmark Card Studio Project File",
    extensions: &["hmk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
