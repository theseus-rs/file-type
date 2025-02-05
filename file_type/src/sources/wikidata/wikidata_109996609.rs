use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109996609: FileFormat = FileFormat {
    id: 109_996_609,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Worksheet, version 9.8 Millennium",
    extensions: &["123"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
