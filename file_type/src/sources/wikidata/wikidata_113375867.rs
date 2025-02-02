use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113375867: FileFormat = FileFormat {
    id: 113_375_867,
    source_type: SourceType::Wikidata,
    name: "Extended GEM Bit Image",
    extensions: &["ximg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
