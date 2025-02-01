use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206381: FileFormat = FileFormat {
    id: 28_206_381,
    puid: "wikidata/28206381",
    name: "VisualBasic form",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
