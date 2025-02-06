use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317315: FileFormat = FileFormat {
    id: 111_317_315,
    source_type: SourceType::Wikidata,
    name: "iPhone ring-tone",
    extensions: &["m4r"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
