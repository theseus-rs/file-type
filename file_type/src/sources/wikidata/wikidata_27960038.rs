use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960038: FileFormat = FileFormat {
    id: 27_960_038,
    source_type: SourceType::Wikidata,
    name: "Windows Media Audio Lossless",
    extensions: &["wma", "wmal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
