use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960004: FileFormat = FileFormat {
    id: 27_960_004,
    source_type: SourceType::Wikidata,
    name: "Real Lossless Codec",
    extensions: &["rmvb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
