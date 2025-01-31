use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207288: FileFormat = FileFormat {
    id: 28_207_288,
    puid: "wikidata/28207288",
    name: "Slow-scan television",
    extensions: &["hrz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
