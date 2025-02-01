use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863736: FileFormat = FileFormat {
    id: 105_863_736,
    puid: "wikidata/105863736",
    name: "MuSiCa text music format (with rem)",
    extensions: &["msd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
