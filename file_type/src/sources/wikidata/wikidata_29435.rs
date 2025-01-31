use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29435: FileFormat = FileFormat {
    id: 29_435,
    puid: "wikidata/29435",
    name: "Dolby TrueHD",
    extensions: &["thd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
