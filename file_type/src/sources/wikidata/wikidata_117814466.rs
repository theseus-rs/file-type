use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117814466: FileFormat = FileFormat {
    id: 117_814_466,
    puid: "wikidata/117814466",
    name: "AdTech perfectfax",
    extensions: &["adt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
