use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131288311: FileFormat = FileFormat {
    id: 131_288_311,
    source_type: SourceType::Wikidata,
    name: "Transaction Execution Approval Language file format",
    extensions: &["teal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
