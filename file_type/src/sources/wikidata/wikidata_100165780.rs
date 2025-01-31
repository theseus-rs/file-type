use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100165780: FileFormat = FileFormat {
    id: 100_165_780,
    puid: "wikidata/100165780",
    name: "Drawing Interchange Format (Binary), version 2018-2021",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
