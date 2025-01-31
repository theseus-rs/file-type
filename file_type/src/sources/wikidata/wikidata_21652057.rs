use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21652057: FileFormat = FileFormat {
    id: 21_652_057,
    puid: "wikidata/21652057",
    name: "Game Cache File",
    extensions: &["gcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
