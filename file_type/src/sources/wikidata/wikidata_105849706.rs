use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849706: FileFormat = FileFormat {
    id: 105_849_706,
    puid: "wikidata/105849706",
    name: "Carbide Create model",
    extensions: &["c2d"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
