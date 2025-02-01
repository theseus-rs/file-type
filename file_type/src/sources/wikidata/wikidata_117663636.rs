use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117663636: FileFormat = FileFormat {
    id: 117_663_636,
    puid: "wikidata/117663636",
    name: "The Print Shop Quick Prints File",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
