use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122069891: FileFormat = FileFormat {
    id: 122_069_891,
    source_type: SourceType::Wikidata,
    name: "Post-It Software Note File",
    extensions: &["ppn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
