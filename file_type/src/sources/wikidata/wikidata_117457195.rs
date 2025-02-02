use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117457195: FileFormat = FileFormat {
    id: 117_457_195,
    source_type: SourceType::Wikidata,
    name: "Raw PIMA SWIR Reflectance Spectral File",
    extensions: &["fos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
