use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205503: FileFormat = FileFormat {
    id: 28_205_503,
    source_type: SourceType::Wikidata,
    name: "GEM resource file",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
