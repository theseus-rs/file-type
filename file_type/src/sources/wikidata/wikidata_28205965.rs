use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205965: FileFormat = FileFormat {
    id: 28_205_965,
    puid: "wikidata/28205965",
    name: "Digital Video Interactive Green Channel",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
