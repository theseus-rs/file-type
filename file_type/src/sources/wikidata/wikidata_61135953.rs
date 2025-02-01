use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61135953: FileFormat = FileFormat {
    id: 61_135_953,
    puid: "wikidata/61135953",
    name: "Drawing Interchange File Format (ASCII), version 2.2",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
