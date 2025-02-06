use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129418045: FileFormat = FileFormat {
    id: 129_418_045,
    source_type: SourceType::Wikidata,
    name: "GoodData-CL script file",
    extensions: &["gdc"],
    media_types: &["text/x-gooddata-cl"],
    signatures: &[],
    related_formats: &[],
};
