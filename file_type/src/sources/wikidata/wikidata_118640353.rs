use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118640353: FileFormat = FileFormat {
    id: 118_640_353,
    source_type: SourceType::Wikidata,
    name: "Picture Definition file",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
