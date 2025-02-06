use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206381: FileFormat = FileFormat {
    id: 28_206_381,
    source_type: SourceType::Wikidata,
    name: "VisualBasic form",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
