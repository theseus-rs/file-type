use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113626475: FileFormat = FileFormat {
    id: 113_626_475,
    source_type: SourceType::Wikidata,
    name: "FOCUS file",
    extensions: &["fex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
