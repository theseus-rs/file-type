use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67384156: FileFormat = FileFormat {
    id: 67_384_156,
    puid: "wikidata/67384156",
    name: "SimLife Animal",
    extensions: &["anl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
