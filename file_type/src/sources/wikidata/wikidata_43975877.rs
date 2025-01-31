use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43975877: FileFormat = FileFormat {
    id: 43_975_877,
    puid: "wikidata/43975877",
    name: "Drawing Interchange File Format (Binary), version 2004-2005",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
