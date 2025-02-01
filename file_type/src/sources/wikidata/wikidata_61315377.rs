use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61315377: FileFormat = FileFormat {
    id: 61_315_377,
    puid: "wikidata/61315377",
    name: "Drawing Interchange File Format (ASCII), version 2.6",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
