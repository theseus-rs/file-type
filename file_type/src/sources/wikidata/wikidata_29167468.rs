use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167468: FileFormat = FileFormat {
    id: 29_167_468,
    puid: "wikidata/29167468",
    name: "Open Virtualization Format Archive Package",
    extensions: &["ova"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
