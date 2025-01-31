use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61901765: FileFormat = FileFormat {
    id: 61_901_765,
    puid: "wikidata/61901765",
    name: "EndNote Import File",
    extensions: &["enr", "enw"],
    media_types: &["application/x-endnote-refer", "application/x-endnote-refer"],
    internal_signatures: &[],
    related_formats: &[],
};
