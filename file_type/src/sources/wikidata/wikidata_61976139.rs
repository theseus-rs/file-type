use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61976139: FileFormat = FileFormat {
    id: 61_976_139,
    puid: "wikidata/61976139",
    name: "Microsoft Visual FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
