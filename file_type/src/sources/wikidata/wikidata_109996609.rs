use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109996609: FileFormat = FileFormat {
    id: 109_996_609,
    puid: "wikidata/109996609",
    name: "Lotus 1-2-3 Worksheet, version 9.8 Millennium",
    extensions: &["123"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
