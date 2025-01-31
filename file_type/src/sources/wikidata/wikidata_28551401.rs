use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551401: FileFormat = FileFormat {
    id: 28_551_401,
    puid: "wikidata/28551401",
    name: "Adobe Separation Table File",
    extensions: &["ast"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
