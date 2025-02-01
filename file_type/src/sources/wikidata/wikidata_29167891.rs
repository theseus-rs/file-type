use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167891: FileFormat = FileFormat {
    id: 29_167_891,
    puid: "wikidata/29167891",
    name: "Personal Ancestral File, version 4",
    extensions: &["paf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
