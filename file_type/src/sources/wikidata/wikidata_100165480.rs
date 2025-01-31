use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100165480: FileFormat = FileFormat {
    id: 100_165_480,
    puid: "wikidata/100165480",
    name: "Drawing Interchange Format (Binary), version 2010-2012",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
