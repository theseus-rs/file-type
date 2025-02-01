use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59851255: FileFormat = FileFormat {
    id: 59_851_255,
    puid: "wikidata/59851255",
    name: "Drawing Interchange File Format (ASCII)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
