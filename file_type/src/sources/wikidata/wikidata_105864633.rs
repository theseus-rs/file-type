use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864633: FileFormat = FileFormat {
    id: 105_864_633,
    source_type: SourceType::Wikidata,
    name: "PiXCL source (with rem)",
    extensions: &["pxl"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
