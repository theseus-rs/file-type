use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61976072: FileFormat = FileFormat {
    id: 61_976_072,
    puid: "wikidata/61976072",
    name: "FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
