use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960038: FileFormat = FileFormat {
    id: 27_960_038,
    source_type: SourceType::Wikidata,
    name: "Windows Media Audio Lossless",
    extensions: &["wma", "wmal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
