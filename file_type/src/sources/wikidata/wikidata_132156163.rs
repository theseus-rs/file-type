use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132156163: FileFormat = FileFormat {
    id: 132_156_163,
    source_type: SourceType::Wikidata,
    name: "NIMAS XML file format",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
