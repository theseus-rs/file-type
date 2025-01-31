use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61053371: FileFormat = FileFormat {
    id: 61_053_371,
    puid: "wikidata/61053371",
    name: "Drawing Interchange File Format (ASCII), version 1.4",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
