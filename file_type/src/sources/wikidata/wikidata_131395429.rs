use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131395429: FileFormat = FileFormat {
    id: 131_395_429,
    puid: "wikidata/131395429",
    name: "Verifpal code",
    extensions: &["vp"],
    media_types: &["text/x-verifpal"],
    internal_signatures: &[],
    related_formats: &[],
};
