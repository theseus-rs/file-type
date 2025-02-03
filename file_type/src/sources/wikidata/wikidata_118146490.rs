use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118146490: FileFormat = FileFormat {
    id: 118_146_490,
    source_type: SourceType::Wikidata,
    name: "Edge-couple Symmetric Stripline File",
    extensions: &["tl5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
