use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100165626: FileFormat = FileFormat {
    id: 100_165_626,
    puid: "wikidata/100165626",
    name: "Drawing Interchange Format (Binary), version 2013-2017",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
