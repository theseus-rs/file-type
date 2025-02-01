use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116331429: FileFormat = FileFormat {
    id: 116_331_429,
    puid: "wikidata/116331429",
    name: "Lawyer File",
    extensions: &["flp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
