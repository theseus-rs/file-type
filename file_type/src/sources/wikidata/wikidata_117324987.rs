use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117324987: FileFormat = FileFormat {
    id: 117_324_987,
    source_type: SourceType::Wikidata,
    name: "LabVIEW control",
    extensions: &["ctl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
