use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5448342: FileFormat = FileFormat {
    id: 5_448_342,
    puid: "wikidata/5448342",
    name: "File change log",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
