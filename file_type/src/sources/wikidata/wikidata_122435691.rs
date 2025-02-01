use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122435691: FileFormat = FileFormat {
    id: 122_435_691,
    puid: "wikidata/122435691",
    name: "NovaBACKUP Job",
    extensions: &["nbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
