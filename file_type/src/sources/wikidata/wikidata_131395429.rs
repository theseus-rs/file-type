use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131395429: FileFormat = FileFormat {
    id: 131_395_429,
    source_type: SourceType::Wikidata,
    name: "Verifpal code",
    extensions: &["vp"],
    media_types: &["text/x-verifpal"],
    signatures: &[],
    related_formats: &[],
};
