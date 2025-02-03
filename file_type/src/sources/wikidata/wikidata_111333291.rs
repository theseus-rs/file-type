use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333291: FileFormat = FileFormat {
    id: 111_333_291,
    source_type: SourceType::Wikidata,
    name: "DisorderTracker2 sample",
    extensions: &["pls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
