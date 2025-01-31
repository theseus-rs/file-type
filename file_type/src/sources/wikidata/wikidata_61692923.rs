use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61692923: FileFormat = FileFormat {
    id: 61_692_923,
    puid: "wikidata/61692923",
    name: "Drawing Interchange File Format (Binary), version R10",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
