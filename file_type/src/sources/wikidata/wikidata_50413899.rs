use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50413899: FileFormat = FileFormat {
    id: 50_413_899,
    puid: "wikidata/50413899",
    name: "Lightwright 3 Show File",
    extensions: &["lw3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
