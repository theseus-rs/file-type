use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122407850: FileFormat = FileFormat {
    id: 122_407_850,
    puid: "wikidata/122407850",
    name: "x86 Symbol File",
    extensions: &["isym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
