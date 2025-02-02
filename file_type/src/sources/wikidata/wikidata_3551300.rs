use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3551300: FileFormat = FileFormat {
    id: 3_551_300,
    source_type: SourceType::Wikidata,
    name: "Universal Subtitle Format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
