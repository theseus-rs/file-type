use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117324994: FileFormat = FileFormat {
    id: 117_324_994,
    source_type: SourceType::Wikidata,
    name: "LabVIEW control template",
    extensions: &["ctt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
