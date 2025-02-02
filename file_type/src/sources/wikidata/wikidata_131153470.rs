use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131153470: FileFormat = FileFormat {
    id: 131_153_470,
    source_type: SourceType::Wikidata,
    name: "sqlite3con file format",
    extensions: &["sqlite3-console"],
    media_types: &["text/x-sqlite3-console"],
    internal_signatures: &[],
    related_formats: &[],
};
