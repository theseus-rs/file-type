use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_19969536: FileFormat = FileFormat {
    id: 19_969_536,
    puid: "wikidata/19969536",
    name: "DSV version 6 format",
    extensions: &["dsv", "dsv6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
