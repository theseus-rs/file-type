use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851415: FileFormat = FileFormat {
    id: 105_851_415,
    puid: "wikidata/105851415",
    name: "T'SoundSystem Source (with rem)",
    extensions: &["tss"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
