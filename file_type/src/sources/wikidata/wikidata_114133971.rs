use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114133971: FileFormat = FileFormat {
    id: 114_133_971,
    source_type: SourceType::Wikidata,
    name: "MSI Molfile",
    extensions: &["msm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
