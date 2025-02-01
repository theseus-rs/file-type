use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116446363: FileFormat = FileFormat {
    id: 116_446_363,
    puid: "wikidata/116446363",
    name: "Work File",
    extensions: &["w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
