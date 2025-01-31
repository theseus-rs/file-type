use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121742883: FileFormat = FileFormat {
    id: 121_742_883,
    puid: "wikidata/121742883",
    name: "The Master Genealogist Project",
    extensions: &["pjc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
