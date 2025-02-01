use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7095914: FileFormat = FileFormat {
    id: 7_095_914,
    puid: "wikidata/7095914",
    name: "OpenXDF",
    extensions: &["xdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
