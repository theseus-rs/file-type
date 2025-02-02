use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117324987: FileFormat = FileFormat {
    id: 117_324_987,
    source_type: SourceType::Wikidata,
    name: "LabVIEW control",
    extensions: &["ctl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
