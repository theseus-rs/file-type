use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131717063: FileFormat = FileFormat {
    id: 131_717_063,
    source_type: SourceType::Wikidata,
    name: "AVS UCD Binary/ASCII file format",
    extensions: &["inp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
