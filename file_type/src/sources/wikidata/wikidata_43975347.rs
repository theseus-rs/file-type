use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43975347: FileFormat = FileFormat {
    id: 43_975_347,
    puid: "wikidata/43975347",
    name: "Drawing Interchange File Format (Binary), version R14",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
