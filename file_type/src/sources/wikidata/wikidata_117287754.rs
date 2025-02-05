use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287754: FileFormat = FileFormat {
    id: 117_287_754,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Notebook File",
    extensions: &["jnb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
