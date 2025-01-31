use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975796: FileFormat = FileFormat {
    id: 28_975_796,
    puid: "wikidata/28975796",
    name: "Drawing Interchange File Format (ASCII), version R10",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
