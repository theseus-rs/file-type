use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55753055: FileFormat = FileFormat {
    id: 55_753_055,
    source_type: SourceType::Wikidata,
    name: "Redcode Metadata File",
    extensions: &["rmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
