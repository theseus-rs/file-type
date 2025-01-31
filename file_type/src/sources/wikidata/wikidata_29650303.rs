use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650303: FileFormat = FileFormat {
    id: 29_650_303,
    puid: "wikidata/29650303",
    name: "PSRFITS",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
