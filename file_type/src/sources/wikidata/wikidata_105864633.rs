use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864633: FileFormat = FileFormat {
    id: 105_864_633,
    source_type: SourceType::Wikidata,
    name: "PiXCL source (with rem)",
    extensions: &["pxl"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
