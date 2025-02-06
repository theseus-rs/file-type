use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116808623: FileFormat = FileFormat {
    id: 116_808_623,
    source_type: SourceType::Wikidata,
    name: "WillMaker File",
    extensions: &["ww8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
