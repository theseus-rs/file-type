use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73624536: FileFormat = FileFormat {
    id: 73_624_536,
    puid: "wikidata/73624536",
    name: "Intuit QuickBooks Backup",
    extensions: &["qbb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
