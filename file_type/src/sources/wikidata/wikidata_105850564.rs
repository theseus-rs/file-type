use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850564: FileFormat = FileFormat {
    id: 105_850_564,
    puid: "wikidata/105850564",
    name: "Camtasia Studio Project (UTF)",
    extensions: &["camproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
