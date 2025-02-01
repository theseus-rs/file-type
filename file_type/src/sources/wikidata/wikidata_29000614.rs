use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000614: FileFormat = FileFormat {
    id: 29_000_614,
    puid: "wikidata/29000614",
    name: "Resource File",
    extensions: &["res"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
