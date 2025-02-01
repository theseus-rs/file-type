use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861359: FileFormat = FileFormat {
    id: 27_861_359,
    puid: "wikidata/27861359",
    name: "Windows Prefetch File, version 30",
    extensions: &["pf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
