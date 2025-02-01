use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128792608: FileFormat = FileFormat {
    id: 128_792_608,
    puid: "wikidata/128792608",
    name: "DAX formula file",
    extensions: &["dax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
