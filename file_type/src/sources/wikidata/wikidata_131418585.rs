use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131418585: FileFormat = FileFormat {
    id: 131_418_585,
    source_type: SourceType::Wikidata,
    name: "wdiff file format",
    extensions: &["wdiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
