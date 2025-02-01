use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64859108: FileFormat = FileFormat {
    id: 64_859_108,
    puid: "wikidata/64859108",
    name: "Family Tree Maker Backup file format",
    extensions: &["fbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
