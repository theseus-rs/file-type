use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51994258: FileFormat = FileFormat {
    id: 51_994_258,
    puid: "wikidata/51994258",
    name: "DEC WPS Plus Document",
    extensions: &["wpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
