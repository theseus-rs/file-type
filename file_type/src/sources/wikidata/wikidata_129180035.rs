use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129180035: FileFormat = FileFormat {
    id: 129_180_035,
    source_type: SourceType::Wikidata,
    name: "Fish shell script",
    extensions: &["fish"],
    media_types: &["application/x-fish"],
    internal_signatures: &[],
    related_formats: &[],
};
