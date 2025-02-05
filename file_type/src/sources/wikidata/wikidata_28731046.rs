use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28731046: FileFormat = FileFormat {
    id: 28_731_046,
    source_type: SourceType::Wikidata,
    name: "APL Transfer File format",
    extensions: &["atf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
