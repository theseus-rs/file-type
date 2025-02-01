use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61131191: FileFormat = FileFormat {
    id: 61_131_191,
    puid: "wikidata/61131191",
    name: "Drawing Interchange File Format (ASCII), version 2",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
