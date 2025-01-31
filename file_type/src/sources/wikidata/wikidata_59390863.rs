use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59390863: FileFormat = FileFormat {
    id: 59_390_863,
    puid: "wikidata/59390863",
    name: "Domino XML Database Export",
    extensions: &["dxl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
