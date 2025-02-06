use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777707: FileFormat = FileFormat {
    id: 28_777_707,
    source_type: SourceType::Wikidata,
    name: "mzML",
    extensions: &["mxml", "mzML", "mzml"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
