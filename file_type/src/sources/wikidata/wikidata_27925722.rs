use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925722: FileFormat = FileFormat {
    id: 27_925_722,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Gazetteer Directory file",
    extensions: &["dir"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
