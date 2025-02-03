use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855115: FileFormat = FileFormat {
    id: 105_855_115,
    source_type: SourceType::Wikidata,
    name: "Akoma Ntoso document",
    extensions: &["xml"],
    media_types: &["application/akn+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
