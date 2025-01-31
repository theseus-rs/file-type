use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4042016: FileFormat = FileFormat {
    id: 4_042_016,
    puid: "wikidata/4042016",
    name: "KSS",
    extensions: &["kss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
