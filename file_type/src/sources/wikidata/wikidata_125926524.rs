use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125926524: FileFormat = FileFormat {
    id: 125_926_524,
    source_type: SourceType::Wikidata,
    name: "Solidworks Drawing Sheet",
    extensions: &["slddrt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
