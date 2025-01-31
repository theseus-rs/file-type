use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131304603: FileFormat = FileFormat {
    id: 131_304_603,
    puid: "wikidata/131304603",
    name: "Transact-SQL file format",
    extensions: &["sql"],
    media_types: &["text/x-tsql"],
    internal_signatures: &[],
    related_formats: &[],
};
