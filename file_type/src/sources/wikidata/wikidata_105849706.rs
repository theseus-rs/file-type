use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849706: FileFormat = FileFormat {
    id: 105_849_706,
    source_type: SourceType::Wikidata,
    name: "Carbide Create model",
    extensions: &["c2d"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
