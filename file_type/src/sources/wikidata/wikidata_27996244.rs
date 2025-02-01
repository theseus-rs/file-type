use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996244: FileFormat = FileFormat {
    id: 27_996_244,
    puid: "wikidata/27996244",
    name: "HyperCard stack",
    extensions: &["pdf", "tif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
