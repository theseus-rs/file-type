use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28449455: FileFormat = FileFormat {
    id: 28_449_455,
    puid: "wikidata/28449455",
    name: "TOML",
    extensions: &["toml"],
    media_types: &["application/toml"],
    internal_signatures: &[],
    related_formats: &[],
};
