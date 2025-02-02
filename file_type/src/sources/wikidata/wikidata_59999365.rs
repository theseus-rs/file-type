use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59999365: FileFormat = FileFormat {
    id: 59_999_365,
    source_type: SourceType::Wikidata,
    name: "Secure DjVU",
    extensions: &["djv", "djvu"],
    media_types: &["image/vnd.djvu", "image/x-djvu"],
    internal_signatures: &[],
    related_formats: &[],
};
