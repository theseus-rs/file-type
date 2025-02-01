use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111665313: FileFormat = FileFormat {
    id: 111_665_313,
    puid: "wikidata/111665313",
    name: "AbiWord Gzip Compressed Document",
    extensions: &["zabw"],
    media_types: &["application/abiword"],
    internal_signatures: &[],
    related_formats: &[],
};
