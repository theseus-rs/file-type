use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860850: FileFormat = FileFormat {
    id: 105_860_850,
    source_type: SourceType::Wikidata,
    name: "SQL Server Reporting Services Report Definition Language",
    extensions: &["rdl", "rdlc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
