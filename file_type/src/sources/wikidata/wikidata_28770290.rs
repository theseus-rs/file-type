use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770290: FileFormat = FileFormat {
    id: 28_770_290,
    source_type: SourceType::Wikidata,
    name: "LSB",
    extensions: &["lsb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
