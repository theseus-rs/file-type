use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124969775: FileFormat = FileFormat {
    id: 124_969_775,
    puid: "wikidata/124969775",
    name: "Songsmith file",
    extensions: &["songsmith"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
