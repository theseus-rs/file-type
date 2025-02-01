use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317248: FileFormat = FileFormat {
    id: 111_317_248,
    puid: "wikidata/111317248",
    name: "Korg Triton or Trinity script file",
    extensions: &["ksc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
