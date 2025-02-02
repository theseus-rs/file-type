use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287754: FileFormat = FileFormat {
    id: 117_287_754,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Notebook File",
    extensions: &["jnb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
