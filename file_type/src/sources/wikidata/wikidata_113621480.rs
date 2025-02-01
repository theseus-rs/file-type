use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113621480: FileFormat = FileFormat {
    id: 113_621_480,
    puid: "wikidata/113621480",
    name: "LoadRunner Analysis file",
    extensions: &["lra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
