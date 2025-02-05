use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_388116: FileFormat = FileFormat {
    id: 388_116,
    source_type: SourceType::Wikidata,
    name: "Electronic Design Interchange Format",
    extensions: &["edf", "edi", "edn", "edo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
