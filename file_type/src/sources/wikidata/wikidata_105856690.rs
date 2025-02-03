use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856690: FileFormat = FileFormat {
    id: 105_856_690,
    source_type: SourceType::Wikidata,
    name: "USeq genome data",
    extensions: &["useq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
