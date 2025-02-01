use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122947132: FileFormat = FileFormat {
    id: 122_947_132,
    puid: "wikidata/122947132",
    name: "Drawing Interchange File Format (ASCII), version 2010-2012",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
