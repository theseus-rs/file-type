use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167884: FileFormat = FileFormat {
    id: 29_167_884,
    puid: "wikidata/29167884",
    name: "Personal Ancestral File",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
