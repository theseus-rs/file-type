use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120867887: FileFormat = FileFormat {
    id: 120_867_887,
    puid: "wikidata/120867887",
    name: "Cumulus Category Exchange File",
    extensions: &["cce"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
