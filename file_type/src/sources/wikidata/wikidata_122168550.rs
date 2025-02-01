use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122168550: FileFormat = FileFormat {
    id: 122_168_550,
    puid: "wikidata/122168550",
    name: "Proactive Password Auditor Project",
    extensions: &["hdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
