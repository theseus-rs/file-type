use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131717063: FileFormat = FileFormat {
    id: 131_717_063,
    source_type: SourceType::Wikidata,
    name: "AVS UCD Binary/ASCII file format",
    extensions: &["inp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
