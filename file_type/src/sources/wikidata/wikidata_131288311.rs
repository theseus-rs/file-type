use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131288311: FileFormat = FileFormat {
    id: 131_288_311,
    source_type: SourceType::Wikidata,
    name: "Transaction Execution Approval Language file format",
    extensions: &["teal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
