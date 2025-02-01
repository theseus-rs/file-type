use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975794: FileFormat = FileFormat {
    id: 28_975_794,
    puid: "wikidata/28975794",
    name: "Drawing Interchange File Format (ASCII), version R11/R12",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
