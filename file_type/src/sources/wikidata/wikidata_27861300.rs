use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861300: FileFormat = FileFormat {
    id: 27_861_300,
    puid: "wikidata/27861300",
    name: "Windows Prefetch File, version 17",
    extensions: &["pf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
