use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122229301: FileFormat = FileFormat {
    id: 122_229_301,
    puid: "wikidata/122229301",
    name: "PGP Whole Disk Encryption",
    extensions: &["wde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
