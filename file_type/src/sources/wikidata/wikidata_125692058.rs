use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692058: FileFormat = FileFormat {
    id: 125_692_058,
    puid: "wikidata/125692058",
    name: "OpenDocument HTML Template file",
    extensions: &["oth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
