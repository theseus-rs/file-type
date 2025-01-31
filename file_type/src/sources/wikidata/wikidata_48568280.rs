use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48568280: FileFormat = FileFormat {
    id: 48_568_280,
    puid: "wikidata/48568280",
    name: "Lightwright 5 Show File",
    extensions: &["lw5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
