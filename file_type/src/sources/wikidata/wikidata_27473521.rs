use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473521: FileFormat = FileFormat {
    id: 27_473_521,
    puid: "wikidata/27473521",
    name: "Advanced Forensic Format, version 2.0",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
