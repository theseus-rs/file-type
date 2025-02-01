use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1260547: FileFormat = FileFormat {
    id: 1_260_547,
    puid: "wikidata/1260547",
    name: "LyRiCs",
    extensions: &["lrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
