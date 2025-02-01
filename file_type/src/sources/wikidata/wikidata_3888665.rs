use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3888665: FileFormat = FileFormat {
    id: 3_888_665,
    puid: "wikidata/3888665",
    name: "Package interchange format",
    extensions: &["pif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
