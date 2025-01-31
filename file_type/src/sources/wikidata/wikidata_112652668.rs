use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652668: FileFormat = FileFormat {
    id: 112_652_668,
    puid: "wikidata/112652668",
    name: "Gold Disk Anim format",
    extensions: &["awm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
