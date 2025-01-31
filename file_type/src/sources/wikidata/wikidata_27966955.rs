use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966955: FileFormat = FileFormat {
    id: 27_966_955,
    puid: "wikidata/27966955",
    name: "USF",
    extensions: &["miniusf", "usf", "usflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
