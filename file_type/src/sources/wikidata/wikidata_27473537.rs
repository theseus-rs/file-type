use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473537: FileFormat = FileFormat {
    id: 27_473_537,
    puid: "wikidata/27473537",
    name: "Advanced Forensic Format, version 3.0",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
