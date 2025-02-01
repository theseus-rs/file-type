use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120042266: FileFormat = FileFormat {
    id: 120_042_266,
    puid: "wikidata/120042266",
    name: "Cheyenne Backup Script",
    extensions: &["asx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
