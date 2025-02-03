use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837148: FileFormat = FileFormat {
    id: 108_837_148,
    source_type: SourceType::Wikidata,
    name: "Quicken Data Backup",
    extensions: &["qdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
