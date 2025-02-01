use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000715: FileFormat = FileFormat {
    id: 29_000_715,
    puid: "wikidata/29000715",
    name: "Unreal model data file",
    extensions: &["3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
