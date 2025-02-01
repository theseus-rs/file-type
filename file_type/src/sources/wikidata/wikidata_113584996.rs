use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113584996: FileFormat = FileFormat {
    id: 113_584_996,
    puid: "wikidata/113584996",
    name: "VideoFactory Project File",
    extensions: &["vf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
