use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111166091: FileFormat = FileFormat {
    id: 111_166_091,
    puid: "wikidata/111166091",
    name: "Ludwig song file",
    extensions: &["ludwig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
