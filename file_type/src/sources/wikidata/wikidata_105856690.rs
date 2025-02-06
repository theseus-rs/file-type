use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856690: FileFormat = FileFormat {
    id: 105_856_690,
    source_type: SourceType::Wikidata,
    name: "USeq genome data",
    extensions: &["useq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
