use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7095768: FileFormat = FileFormat {
    id: 7_095_768,
    source_type: SourceType::Wikidata,
    name: "OpenDRIVE",
    extensions: &["xodr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
