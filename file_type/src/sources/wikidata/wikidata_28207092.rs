use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207092: FileFormat = FileFormat {
    id: 28_207_092,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Index file",
    extensions: &["sdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
