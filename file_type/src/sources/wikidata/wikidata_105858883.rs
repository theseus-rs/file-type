use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858883: FileFormat = FileFormat {
    id: 105_858_883,
    puid: "wikidata/105858883",
    name: "Dore Raster bitmap (with rem)",
    extensions: &["dore", "img"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
