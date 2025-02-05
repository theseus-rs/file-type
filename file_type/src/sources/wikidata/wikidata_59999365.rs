use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59999365: FileFormat = FileFormat {
    id: 59_999_365,
    source_type: SourceType::Wikidata,
    name: "Secure DjVU",
    extensions: &["djv", "djvu"],
    media_types: &["image/vnd.djvu", "image/x-djvu"],
    signatures: &[],
    related_formats: &[],
};
