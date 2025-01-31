use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61053201: FileFormat = FileFormat {
    id: 61_053_201,
    puid: "wikidata/61053201",
    name: "Drawing Interchange File Format (ASCII), version 1.3",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
