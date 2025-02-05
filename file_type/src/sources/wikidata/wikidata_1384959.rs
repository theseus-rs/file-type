use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1384959: FileFormat = FileFormat {
    id: 1_384_959,
    source_type: SourceType::Wikidata,
    name: "Extensible Forms Description Language",
    extensions: &["frm", "xfd", "xfdl"],
    media_types: &["application/vnd.xfdl"],
    signatures: &[],
    related_formats: &[],
};
