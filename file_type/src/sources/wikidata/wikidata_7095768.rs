use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7095768: FileFormat = FileFormat {
    id: 7_095_768,
    puid: "wikidata/7095768",
    name: "OpenDRIVE",
    extensions: &["xodr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
