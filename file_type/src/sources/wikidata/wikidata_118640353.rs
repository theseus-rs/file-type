use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118640353: FileFormat = FileFormat {
    id: 118_640_353,
    source_type: SourceType::Wikidata,
    name: "Picture Definition file",
    extensions: &["lpd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
