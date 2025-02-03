use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849994: FileFormat = FileFormat {
    id: 105_849_994,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM C-Crypt protected (v1.02)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
