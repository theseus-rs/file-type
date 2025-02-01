use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58335745: FileFormat = FileFormat {
    id: 58_335_745,
    puid: "wikidata/58335745",
    name: "Acrobat Catalog Cat File",
    extensions: &["cat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
