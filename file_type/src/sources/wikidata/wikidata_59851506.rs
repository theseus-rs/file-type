use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59851506: FileFormat = FileFormat {
    id: 59_851_506,
    source_type: SourceType::Wikidata,
    name: "DROID File Collection File Format",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
