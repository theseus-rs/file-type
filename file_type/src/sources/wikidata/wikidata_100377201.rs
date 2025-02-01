use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100377201: FileFormat = FileFormat {
    id: 100_377_201,
    puid: "wikidata/100377201",
    name: "HP TRIM Outlook Saved Message File",
    extensions: &["vmbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
