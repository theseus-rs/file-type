use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238259: FileFormat = FileFormat {
    id: 110_238_259,
    source_type: SourceType::Wikidata,
    name: "Dramatica/StoryView Exchange",
    extensions: &["dsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
