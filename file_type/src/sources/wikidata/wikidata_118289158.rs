use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118289158: FileFormat = FileFormat {
    id: 118_289_158,
    source_type: SourceType::Wikidata,
    name: "Collection File",
    extensions: &["cfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
