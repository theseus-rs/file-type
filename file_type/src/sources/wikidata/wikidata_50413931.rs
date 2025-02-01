use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50413931: FileFormat = FileFormat {
    id: 50_413_931,
    puid: "wikidata/50413931",
    name: "Lightwright 2 Show File",
    extensions: &["lw2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
