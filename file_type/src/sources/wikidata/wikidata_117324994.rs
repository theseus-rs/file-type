use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117324994: FileFormat = FileFormat {
    id: 117_324_994,
    source_type: SourceType::Wikidata,
    name: "LabVIEW control template",
    extensions: &["ctt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
