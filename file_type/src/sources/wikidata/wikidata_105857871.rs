use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857871: FileFormat = FileFormat {
    id: 105_857_871,
    puid: "wikidata/105857871",
    name: "ILINK linker Configuration",
    extensions: &["icf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
