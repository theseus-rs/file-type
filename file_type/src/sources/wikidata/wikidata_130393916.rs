use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130393916: FileFormat = FileFormat {
    id: 130_393_916,
    source_type: SourceType::Wikidata,
    name: "Actual Drawing file",
    extensions: &["adf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
