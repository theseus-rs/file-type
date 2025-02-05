use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129180035: FileFormat = FileFormat {
    id: 129_180_035,
    source_type: SourceType::Wikidata,
    name: "Fish shell script",
    extensions: &["fish"],
    media_types: &["application/x-fish"],
    signatures: &[],
    related_formats: &[],
};
