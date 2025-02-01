use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49800136: FileFormat = FileFormat {
    id: 49_800_136,
    puid: "wikidata/49800136",
    name: "Vectorworks file format, version 12.5",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[],
    related_formats: &[],
};
