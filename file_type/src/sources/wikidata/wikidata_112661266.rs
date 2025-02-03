use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661266: FileFormat = FileFormat {
    id: 112_661_266,
    source_type: SourceType::Wikidata,
    name: "Lightscape Preparation File",
    extensions: &["lp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
