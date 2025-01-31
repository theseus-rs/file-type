use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27861323: FileFormat = FileFormat {
    id: 27_861_323,
    puid: "wikidata/27861323",
    name: "Windows Prefetch File, version 23",
    extensions: &["pf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
