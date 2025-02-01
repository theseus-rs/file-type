use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111393762: FileFormat = FileFormat {
    id: 111_393_762,
    puid: "wikidata/111393762",
    name: "Database Oasis Template",
    extensions: &["mkt", "mktx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
