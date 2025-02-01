use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61315572: FileFormat = FileFormat {
    id: 61_315_572,
    puid: "wikidata/61315572",
    name: "Drawing Interchange File Format (ASCII), version R9",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
