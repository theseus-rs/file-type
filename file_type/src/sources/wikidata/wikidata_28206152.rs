use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206152: FileFormat = FileFormat {
    id: 28_206_152,
    puid: "wikidata/28206152",
    name: "FSH",
    extensions: &["fsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
