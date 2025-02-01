use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28790258: FileFormat = FileFormat {
    id: 28_790_258,
    puid: "wikidata/28790258",
    name: "maz",
    extensions: &["maz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
