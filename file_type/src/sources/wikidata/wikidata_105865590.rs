use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865590: FileFormat = FileFormat {
    id: 105_865_590,
    source_type: SourceType::Wikidata,
    name: "ProPresenter 6 presentation",
    extensions: &["pro6"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
