use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473397: FileFormat = FileFormat {
    id: 27_473_397,
    puid: "wikidata/27473397",
    name: "Advanced Forensic Format, version 1.0",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
