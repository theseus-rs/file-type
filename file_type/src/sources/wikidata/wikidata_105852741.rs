use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852741: FileFormat = FileFormat {
    id: 105_852_741,
    source_type: SourceType::Wikidata,
    name: "Snagit Windows Profile",
    extensions: &["snagprof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
