use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122168574: FileFormat = FileFormat {
    id: 122_168_574,
    puid: "wikidata/122168574",
    name: "Old Security Explorer Project",
    extensions: &["nsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
