use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063295: FileFormat = FileFormat {
    id: 52_063_295,
    source_type: SourceType::Wikidata,
    name: "SAS for MS-DOS Database",
    extensions: &["ssd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
