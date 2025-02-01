use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65533770: FileFormat = FileFormat {
    id: 65_533_770,
    puid: "wikidata/65533770",
    name: "Open Recipe Format",
    extensions: &["orf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
