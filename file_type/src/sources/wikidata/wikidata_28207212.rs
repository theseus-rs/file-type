use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207212: FileFormat = FileFormat {
    id: 28_207_212,
    source_type: SourceType::Wikidata,
    name: "Quantel VPB image",
    extensions: &["vpb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
