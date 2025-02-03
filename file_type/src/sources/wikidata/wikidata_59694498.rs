use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59694498: FileFormat = FileFormat {
    id: 59_694_498,
    source_type: SourceType::Wikidata,
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
