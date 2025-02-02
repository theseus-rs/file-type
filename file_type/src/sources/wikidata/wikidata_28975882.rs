use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975882: FileFormat = FileFormat {
    id: 28_975_882,
    source_type: SourceType::Wikidata,
    name: "SOLIDWORKS Assembly",
    extensions: &["sldasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
