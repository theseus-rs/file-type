use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116784985: FileFormat = FileFormat {
    id: 116_784_985,
    puid: "wikidata/116784985",
    name: "Project Manager Pro Chart file",
    extensions: &["pmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
