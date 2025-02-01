use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43974596: FileFormat = FileFormat {
    id: 43_974_596,
    puid: "wikidata/43974596",
    name: "Drawing Interchange File Format (Binary), version R13",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
