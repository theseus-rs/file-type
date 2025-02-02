use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856497: FileFormat = FileFormat {
    id: 105_856_497,
    source_type: SourceType::Wikidata,
    name: "Khoros Visual Programming Workspace (with rem)",
    extensions: &["wk"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
