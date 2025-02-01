use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122403479: FileFormat = FileFormat {
    id: 122_403_479,
    puid: "wikidata/122403479",
    name: "CodeWarrior Resource File",
    extensions: &["rsrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
