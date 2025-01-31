use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113626396: FileFormat = FileFormat {
    id: 113_626_396,
    puid: "wikidata/113626396",
    name: "ScatterShow file",
    extensions: &["scattershow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
