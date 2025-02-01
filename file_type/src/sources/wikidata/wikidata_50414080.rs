use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50414080: FileFormat = FileFormat {
    id: 50_414_080,
    puid: "wikidata/50414080",
    name: "Lightwright 6 Show File",
    extensions: &["lw6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
