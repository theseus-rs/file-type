use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113557082: FileFormat = FileFormat {
    id: 113_557_082,
    puid: "wikidata/113557082",
    name: "Creator Image format",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
