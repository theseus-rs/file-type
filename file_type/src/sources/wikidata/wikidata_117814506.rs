use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117814506: FileFormat = FileFormat {
    id: 117_814_506,
    puid: "wikidata/117814506",
    name: "Adaptive Information Systems",
    extensions: &["ais"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
