use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105822792: FileFormat = FileFormat {
    id: 105_822_792,
    source_type: SourceType::Wikidata,
    name: "AMDIS Target Compounds Library",
    extensions: &["MSL"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
