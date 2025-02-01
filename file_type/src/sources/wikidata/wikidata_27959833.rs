use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959833: FileFormat = FileFormat {
    id: 27_959_833,
    puid: "wikidata/27959833",
    name: "Cool Edit/Audition Multi Track Session file",
    extensions: &["ses"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
