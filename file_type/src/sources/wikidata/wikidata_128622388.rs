use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128622388: FileFormat = FileFormat {
    id: 128_622_388,
    puid: "wikidata/128622388",
    name: "Augeas file format",
    extensions: &["aug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
