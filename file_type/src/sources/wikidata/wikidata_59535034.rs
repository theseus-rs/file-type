use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59535034: FileFormat = FileFormat {
    id: 59_535_034,
    puid: "wikidata/59535034",
    name: "Stuffit Archive File",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    internal_signatures: &[],
    related_formats: &[],
};
