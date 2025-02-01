use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55739333: FileFormat = FileFormat {
    id: 55_739_333,
    puid: "wikidata/55739333",
    name: "CRAM file format, version 2",
    extensions: &["cram"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
