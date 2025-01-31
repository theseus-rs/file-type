use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1924866: FileFormat = FileFormat {
    id: 1_924_866,
    puid: "wikidata/1924866",
    name: "Metalink",
    extensions: &["meta4", "metalink"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
