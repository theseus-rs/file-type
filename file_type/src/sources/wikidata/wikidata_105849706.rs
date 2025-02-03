use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849706: FileFormat = FileFormat {
    id: 105_849_706,
    source_type: SourceType::Wikidata,
    name: "Carbide Create model",
    extensions: &["c2d"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
