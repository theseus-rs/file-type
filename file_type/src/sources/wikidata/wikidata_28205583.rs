use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205583: FileFormat = FileFormat {
    id: 28_205_583,
    puid: "wikidata/28205583",
    name: "OS/2 Pointer",
    extensions: &["ptr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
