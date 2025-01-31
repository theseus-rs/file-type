use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110977953: FileFormat = FileFormat {
    id: 110_977_953,
    puid: "wikidata/110977953",
    name: "AutoDesk 16-bit Animation File",
    extensions: &["flx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
