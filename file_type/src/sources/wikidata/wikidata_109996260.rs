use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109996260: FileFormat = FileFormat {
    id: 109_996_260,
    puid: "wikidata/109996260",
    name: "Lotus 1-2-3 Worksheet, version 97",
    extensions: &["123"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
