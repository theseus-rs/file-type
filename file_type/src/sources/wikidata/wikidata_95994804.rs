use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95994804: FileFormat = FileFormat {
    id: 95_994_804,
    puid: "wikidata/95994804",
    name: "Spatial Data Transfer Standard format",
    extensions: &["ddf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
