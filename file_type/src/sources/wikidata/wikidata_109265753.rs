use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109265753: FileFormat = FileFormat {
    id: 109_265_753,
    source_type: SourceType::Wikidata,
    name: "PagePlus Template",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
