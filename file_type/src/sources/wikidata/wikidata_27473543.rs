use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473543: FileFormat = FileFormat {
    id: 27_473_543,
    puid: "wikidata/27473543",
    name: "Advanced Forensic Format, version 4",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
