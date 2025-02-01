use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120867987: FileFormat = FileFormat {
    id: 120_867_987,
    puid: "wikidata/120867987",
    name: "Cumulus Catalog File",
    extensions: &["ccf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
