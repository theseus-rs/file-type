use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114133971: FileFormat = FileFormat {
    id: 114_133_971,
    source_type: SourceType::Wikidata,
    name: "MSI Molfile",
    extensions: &["msm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
