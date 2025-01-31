use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111392536: FileFormat = FileFormat {
    id: 111_392_536,
    puid: "wikidata/111392536",
    name: "Bryce 5 File",
    extensions: &["br5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
