use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205992: FileFormat = FileFormat {
    id: 28_205_992,
    puid: "wikidata/28205992",
    name: "Digital Video Interactive Alpha Channel",
    extensions: &["ima"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
