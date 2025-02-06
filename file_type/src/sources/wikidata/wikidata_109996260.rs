use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109996260: FileFormat = FileFormat {
    id: 109_996_260,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Worksheet, version 97",
    extensions: &["123"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
