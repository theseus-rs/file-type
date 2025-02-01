use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205925: FileFormat = FileFormat {
    id: 28_205_925,
    puid: "wikidata/28205925",
    name: "Doodle",
    extensions: &["doo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
