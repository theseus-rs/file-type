use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72959001: FileFormat = FileFormat {
    id: 72_959_001,
    puid: "wikidata/72959001",
    name: "PrintArtist project",
    extensions: &["pa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
