use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979502: FileFormat = FileFormat {
    id: 27_979_502,
    source_type: SourceType::Wikidata,
    name: "DNG camera profile",
    extensions: &["dcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
