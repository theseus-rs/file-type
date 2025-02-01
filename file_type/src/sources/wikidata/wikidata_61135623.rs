use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61135623: FileFormat = FileFormat {
    id: 61_135_623,
    puid: "wikidata/61135623",
    name: "Drawing Interchange File Format (ASCII), version 2.1",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
