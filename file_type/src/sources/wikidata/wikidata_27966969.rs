use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966969: FileFormat = FileFormat {
    id: 27_966_969,
    puid: "wikidata/27966969",
    name: "Crystal Caves Sound format",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
