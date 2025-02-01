use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206272: FileFormat = FileFormat {
    id: 28_206_272,
    puid: "wikidata/28206272",
    name: "HTC splashscreen",
    extensions: &["img", "nb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
