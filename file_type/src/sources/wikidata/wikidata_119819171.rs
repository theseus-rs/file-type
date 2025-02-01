use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119819171: FileFormat = FileFormat {
    id: 119_819_171,
    puid: "wikidata/119819171",
    name: "Final Draft AV Template",
    extensions: &["xavt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
