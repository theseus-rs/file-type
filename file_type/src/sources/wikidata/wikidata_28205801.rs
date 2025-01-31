use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205801: FileFormat = FileFormat {
    id: 28_205_801,
    puid: "wikidata/28205801",
    name: "IMG Picture Format",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
