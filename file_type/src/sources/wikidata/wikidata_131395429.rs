use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131395429: FileFormat = FileFormat {
    id: 131_395_429,
    source_type: SourceType::Wikidata,
    name: "Verifpal code",
    extensions: &["vp"],
    media_types: &["text/x-verifpal"],
    internal_signatures: &[],
    related_formats: &[],
};
