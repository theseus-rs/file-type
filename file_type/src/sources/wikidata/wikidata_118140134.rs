use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118140134: FileFormat = FileFormat {
    id: 118_140_134,
    source_type: SourceType::Wikidata,
    name: "Serenade Project File",
    extensions: &["ssp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
