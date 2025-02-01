use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967185: FileFormat = FileFormat {
    id: 27_967_185,
    puid: "wikidata/27967185",
    name: "Fuchs Tracker",
    extensions: &["fchs", "ft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
