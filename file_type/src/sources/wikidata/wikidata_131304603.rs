use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131304603: FileFormat = FileFormat {
    id: 131_304_603,
    source_type: SourceType::Wikidata,
    name: "Transact-SQL file format",
    extensions: &["sql"],
    media_types: &["text/x-tsql"],
    signatures: &[],
    related_formats: &[],
};
