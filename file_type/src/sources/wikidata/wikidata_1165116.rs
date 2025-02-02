use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1165116: FileFormat = FileFormat {
    id: 1_165_116,
    source_type: SourceType::Wikidata,
    name: "Perl module",
    extensions: &["pm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
