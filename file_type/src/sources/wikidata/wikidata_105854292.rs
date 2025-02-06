use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854292: FileFormat = FileFormat {
    id: 105_854_292,
    source_type: SourceType::Wikidata,
    name: "Quest Adventure Script (v5)",
    extensions: &["aslx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
