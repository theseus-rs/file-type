use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837148: FileFormat = FileFormat {
    id: 108_837_148,
    source_type: SourceType::Wikidata,
    name: "Quicken Data Backup",
    extensions: &["qdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
