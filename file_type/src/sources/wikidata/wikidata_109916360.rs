use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109916360: FileFormat = FileFormat {
    id: 109_916_360,
    puid: "wikidata/109916360",
    name: "JMP Data Table",
    extensions: &["jmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
