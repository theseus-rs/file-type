use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771272: FileFormat = FileFormat {
    id: 28_771_272,
    puid: "wikidata/28771272",
    name: "MVG",
    extensions: &["mvg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
