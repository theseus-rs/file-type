use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55739342: FileFormat = FileFormat {
    id: 55_739_342,
    puid: "wikidata/55739342",
    name: "CRAM file format, version 3",
    extensions: &["cram"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
