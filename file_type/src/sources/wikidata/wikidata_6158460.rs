use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6158460: FileFormat = FileFormat {
    id: 6_158_460,
    source_type: SourceType::Wikidata,
    name: "Video Recording Object file",
    extensions: &["vro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
