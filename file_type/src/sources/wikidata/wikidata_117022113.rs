use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117022113: FileFormat = FileFormat {
    id: 117_022_113,
    puid: "wikidata/117022113",
    name: "Garden File",
    extensions: &["grd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
