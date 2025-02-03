use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849583: FileFormat = FileFormat {
    id: 105_849_583,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Crack Soft's cryptor encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
