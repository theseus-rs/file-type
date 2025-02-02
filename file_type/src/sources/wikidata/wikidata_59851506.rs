use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59851506: FileFormat = FileFormat {
    id: 59_851_506,
    source_type: SourceType::Wikidata,
    name: "DROID File Collection File Format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
