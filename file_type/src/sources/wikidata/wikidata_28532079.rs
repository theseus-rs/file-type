use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28532079: FileFormat = FileFormat {
    id: 28_532_079,
    puid: "wikidata/28532079",
    name: "Alchemy Format",
    extensions: &["alc"],
    media_types: &["chemical/x-alchemy"],
    internal_signatures: &[],
    related_formats: &[],
};
