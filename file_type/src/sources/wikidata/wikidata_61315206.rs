use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61315206: FileFormat = FileFormat {
    id: 61_315_206,
    puid: "wikidata/61315206",
    name: "Drawing Interchange File Format (ASCII), version 2.5",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
