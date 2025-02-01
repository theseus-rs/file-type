use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063295: FileFormat = FileFormat {
    id: 52_063_295,
    puid: "wikidata/52063295",
    name: "SAS for MS-DOS Database",
    extensions: &["ssd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
