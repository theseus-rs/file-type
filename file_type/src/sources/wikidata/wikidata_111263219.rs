use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263219: FileFormat = FileFormat {
    id: 111_263_219,
    puid: "wikidata/111263219",
    name: "DCM module",
    extensions: &["dcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
