use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130266674: FileFormat = FileFormat {
    id: 130_266_674,
    puid: "wikidata/130266674",
    name: "Luau source code file",
    extensions: &["luau"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
