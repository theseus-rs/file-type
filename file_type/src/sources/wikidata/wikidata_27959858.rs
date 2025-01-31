use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959858: FileFormat = FileFormat {
    id: 27_959_858,
    puid: "wikidata/27959858",
    name: "Make-A-Melody song file",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
