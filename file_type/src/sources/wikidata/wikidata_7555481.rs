use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7555481: FileFormat = FileFormat {
    id: 7_555_481,
    source_type: SourceType::Wikidata,
    name: "sol",
    extensions: &["sol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
