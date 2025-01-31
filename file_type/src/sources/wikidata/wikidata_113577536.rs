use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113577536: FileFormat = FileFormat {
    id: 113_577_536,
    puid: "wikidata/113577536",
    name: "WinOnCD Image",
    extensions: &["c2d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
