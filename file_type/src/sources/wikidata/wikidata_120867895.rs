use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120867895: FileFormat = FileFormat {
    id: 120_867_895,
    puid: "wikidata/120867895",
    name: "Cumulus Record Exchange File",
    extensions: &["cre"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
