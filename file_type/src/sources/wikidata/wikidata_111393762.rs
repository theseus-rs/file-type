use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111393762: FileFormat = FileFormat {
    id: 111_393_762,
    source_type: SourceType::Wikidata,
    name: "Database Oasis Template",
    extensions: &["mkt", "mktx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
