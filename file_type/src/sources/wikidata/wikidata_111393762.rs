use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111393762: FileFormat = FileFormat {
    id: 111_393_762,
    source_type: SourceType::Wikidata,
    name: "Database Oasis Template",
    extensions: &["mkt", "mktx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
