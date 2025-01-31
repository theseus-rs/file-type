use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124158014: FileFormat = FileFormat {
    id: 124_158_014,
    puid: "wikidata/124158014",
    name: "Drawing Interchange File Format (ASCII), version 2007-2008-2009",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
