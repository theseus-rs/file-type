use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113487224: FileFormat = FileFormat {
    id: 113_487_224,
    puid: "wikidata/113487224",
    name: "Persuasion Player File 3",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
