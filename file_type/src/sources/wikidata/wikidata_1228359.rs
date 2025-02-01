use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1228359: FileFormat = FileFormat {
    id: 1_228_359,
    puid: "wikidata/1228359",
    name: "Disc Description Protocol",
    extensions: &["ddp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
