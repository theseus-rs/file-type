use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856497: FileFormat = FileFormat {
    id: 105_856_497,
    source_type: SourceType::Wikidata,
    name: "Khoros Visual Programming Workspace (with rem)",
    extensions: &["wk"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
