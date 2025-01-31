use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207293: FileFormat = FileFormat {
    id: 28_207_293,
    puid: "wikidata/28207293",
    name: "Softimage PIC",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
