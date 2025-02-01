use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28731046: FileFormat = FileFormat {
    id: 28_731_046,
    puid: "wikidata/28731046",
    name: "APL Transfer File format",
    extensions: &["atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
