use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_93275504: FileFormat = FileFormat {
    id: 93_275_504,
    puid: "wikidata/93275504",
    name: "Procreate",
    extensions: &["procreate"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
