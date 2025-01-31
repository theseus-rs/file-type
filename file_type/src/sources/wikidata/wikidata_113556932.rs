use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113556932: FileFormat = FileFormat {
    id: 113_556_932,
    puid: "wikidata/113556932",
    name: "Duplicator Image format",
    extensions: &["dao"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
