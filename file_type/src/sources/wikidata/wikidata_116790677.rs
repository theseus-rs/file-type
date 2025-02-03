use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116790677: FileFormat = FileFormat {
    id: 116_790_677,
    source_type: SourceType::Wikidata,
    name: "Prepress File",
    extensions: &["sep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
