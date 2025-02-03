use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131304603: FileFormat = FileFormat {
    id: 131_304_603,
    source_type: SourceType::Wikidata,
    name: "Transact-SQL file format",
    extensions: &["sql"],
    media_types: &["text/x-tsql"],
    internal_signatures: &[],
    related_formats: &[],
};
