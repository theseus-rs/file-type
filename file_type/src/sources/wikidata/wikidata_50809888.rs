use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50809888: FileFormat = FileFormat {
    id: 50_809_888,
    puid: "wikidata/50809888",
    name: "Google Document Link File",
    extensions: &[
        "gdoc", "gdraw", "gform", "gmap", "gsheet", "gsite", "gslides",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
