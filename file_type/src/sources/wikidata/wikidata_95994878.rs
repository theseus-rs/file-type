use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95994878: FileFormat = FileFormat {
    id: 95_994_878,
    puid: "wikidata/95994878",
    name: "Canadian digital elevation data format",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
