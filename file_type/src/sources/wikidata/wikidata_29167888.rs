use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167888: FileFormat = FileFormat {
    id: 29_167_888,
    puid: "wikidata/29167888",
    name: "Personal Ancestral File, version 3",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
