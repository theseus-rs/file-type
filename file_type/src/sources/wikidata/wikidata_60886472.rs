use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60886472: FileFormat = FileFormat {
    id: 60_886_472,
    puid: "wikidata/60886472",
    name: "Drawing Interchange File Format (ASCII), version 1",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
