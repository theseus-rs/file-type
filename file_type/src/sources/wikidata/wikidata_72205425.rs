use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72205425: FileFormat = FileFormat {
    id: 72_205_425,
    puid: "wikidata/72205425",
    name: "Exchange Offline Address Book",
    extensions: &["lzx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
