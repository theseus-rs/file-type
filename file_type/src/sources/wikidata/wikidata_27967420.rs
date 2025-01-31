use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967420: FileFormat = FileFormat {
    id: 27_967_420,
    puid: "wikidata/27967420",
    name: "ANSI Music",
    extensions: &["ams", "mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
