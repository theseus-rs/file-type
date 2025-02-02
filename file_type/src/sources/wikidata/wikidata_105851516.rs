use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851516: FileFormat = FileFormat {
    id: 105_851_516,
    source_type: SourceType::Wikidata,
    name: "LaTeX 2e document (with rem)",
    extensions: &["tex"],
    media_types: &["application/x-tex"],
    internal_signatures: &[],
    related_formats: &[],
};
