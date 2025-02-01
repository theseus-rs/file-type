use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131153470: FileFormat = FileFormat {
    id: 131_153_470,
    puid: "wikidata/131153470",
    name: "sqlite3con file format",
    extensions: &["sqlite3-console"],
    media_types: &["text/x-sqlite3-console"],
    internal_signatures: &[],
    related_formats: &[],
};
