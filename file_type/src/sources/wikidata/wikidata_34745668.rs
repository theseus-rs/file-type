use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34745668: FileFormat = FileFormat {
    id: 34_745_668,
    puid: "wikidata/34745668",
    name: "Squeeze",
    extensions: &["qqq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
