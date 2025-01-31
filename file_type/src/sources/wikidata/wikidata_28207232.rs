use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207232: FileFormat = FileFormat {
    id: 28_207_232,
    puid: "wikidata/28207232",
    name: "RLA",
    extensions: &["rla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
