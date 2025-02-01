use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43975668: FileFormat = FileFormat {
    id: 43_975_668,
    puid: "wikidata/43975668",
    name: "Drawing Interchange File Format (Binary), version 2000-2002",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
