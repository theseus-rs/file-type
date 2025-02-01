use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84996757: FileFormat = FileFormat {
    id: 84_996_757,
    puid: "wikidata/84996757",
    name: "HP Photo Album",
    extensions: &["albm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
