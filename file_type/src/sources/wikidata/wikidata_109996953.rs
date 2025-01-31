use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109996953: FileFormat = FileFormat {
    id: 109_996_953,
    puid: "wikidata/109996953",
    name: "Autocad DMP File",
    extensions: &["dmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
