use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864633: FileFormat = FileFormat {
    id: 105_864_633,
    puid: "wikidata/105864633",
    name: "PiXCL source (with rem)",
    extensions: &["pxl"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
