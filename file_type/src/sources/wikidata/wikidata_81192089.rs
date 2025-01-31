use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81192089: FileFormat = FileFormat {
    id: 81_192_089,
    puid: "wikidata/81192089",
    name: "Infinity Engine Compiled Script",
    extensions: &["bcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
