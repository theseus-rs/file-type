use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47246032: FileFormat = FileFormat {
    id: 47_246_032,
    puid: "wikidata/47246032",
    name: "PowerVR Object Data",
    extensions: &["pod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
