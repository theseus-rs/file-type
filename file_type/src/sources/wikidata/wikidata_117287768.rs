use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117287768: FileFormat = FileFormat {
    id: 117_287_768,
    source_type: SourceType::Wikidata,
    name: "SigmaPlot Template File",
    extensions: &["jnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
