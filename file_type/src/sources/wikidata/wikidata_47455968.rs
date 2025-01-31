use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47455968: FileFormat = FileFormat {
    id: 47_455_968,
    puid: "wikidata/47455968",
    name: "SafeGuard Encrypted Virtual Disk",
    extensions: &["hdr", "vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
