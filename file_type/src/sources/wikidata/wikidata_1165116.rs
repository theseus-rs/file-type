use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1165116: FileFormat = FileFormat {
    id: 1_165_116,
    source_type: SourceType::Wikidata,
    name: "Perl module",
    extensions: &["pm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
