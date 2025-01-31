use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114237069: FileFormat = FileFormat {
    id: 114_237_069,
    puid: "wikidata/114237069",
    name: "Visual C++ Definition format",
    extensions: &["def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
