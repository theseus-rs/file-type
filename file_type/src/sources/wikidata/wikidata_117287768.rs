use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287768: FileFormat = FileFormat {
    id: 117_287_768,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Template File",
    extensions: &["jnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
