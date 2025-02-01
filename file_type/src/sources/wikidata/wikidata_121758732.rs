use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121758732: FileFormat = FileFormat {
    id: 121_758_732,
    puid: "wikidata/121758732",
    name: "Family Tree Maker FTMB Backup File",
    extensions: &["ftmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
