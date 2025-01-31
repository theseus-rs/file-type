use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61578345: FileFormat = FileFormat {
    id: 61_578_345,
    puid: "wikidata/61578345",
    name: "Drawing Interchange File Format (ASCII), version 2000-2002",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
