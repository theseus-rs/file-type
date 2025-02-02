use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473679: FileFormat = FileFormat {
    id: 27_473_679,
    source_type: SourceType::Wikidata,
    name: "Band Sequential Image File",
    extensions: &["bsq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
