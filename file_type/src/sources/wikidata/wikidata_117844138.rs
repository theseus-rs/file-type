use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117844138: FileFormat = FileFormat {
    id: 117_844_138,
    puid: "wikidata/117844138",
    name: "Hayes JTFax file",
    extensions: &["jtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
