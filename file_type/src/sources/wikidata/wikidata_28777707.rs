use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777707: FileFormat = FileFormat {
    id: 28_777_707,
    source_type: SourceType::Wikidata,
    name: "mzML",
    extensions: &["mxml", "mzML", "mzml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
