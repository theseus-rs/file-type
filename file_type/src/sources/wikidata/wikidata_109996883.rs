use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109996883: FileFormat = FileFormat {
    id: 109_996_883,
    puid: "wikidata/109996883",
    name: "Primavera P6 Project Management XER File",
    extensions: &["xer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
