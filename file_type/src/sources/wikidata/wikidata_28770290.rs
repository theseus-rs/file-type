use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770290: FileFormat = FileFormat {
    id: 28_770_290,
    source_type: SourceType::Wikidata,
    name: "LSB",
    extensions: &["lsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
