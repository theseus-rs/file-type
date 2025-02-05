use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333291: FileFormat = FileFormat {
    id: 111_333_291,
    source_type: SourceType::Wikidata,
    name: "DisorderTracker2 sample",
    extensions: &["pls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
