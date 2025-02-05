use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59694498: FileFormat = FileFormat {
    id: 59_694_498,
    source_type: SourceType::Wikidata,
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
