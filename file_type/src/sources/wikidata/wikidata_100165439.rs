use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100165439: FileFormat = FileFormat {
    id: 100_165_439,
    puid: "wikidata/100165439",
    name: "Drawing Interchange Format (Binary), version 2007-2009",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
