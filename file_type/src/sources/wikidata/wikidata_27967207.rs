use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967207: FileFormat = FileFormat {
    id: 27_967_207,
    source_type: SourceType::Wikidata,
    name: "Pro Tracker v1.xx module",
    extensions: &["pt1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
