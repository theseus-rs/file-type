use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59693916: FileFormat = FileFormat {
    id: 59_693_916,
    puid: "wikidata/59693916",
    name: "QuadriSpace format",
    extensions: &["qsd", "qsl", "qsm", "qst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
