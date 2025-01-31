use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60887256: FileFormat = FileFormat {
    id: 60_887_256,
    puid: "wikidata/60887256",
    name: "Drawing Interchange File Format (ASCII), version 1.1",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
