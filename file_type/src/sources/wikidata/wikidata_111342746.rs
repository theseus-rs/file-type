use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342746: FileFormat = FileFormat {
    id: 111_342_746,
    source_type: SourceType::Wikidata,
    name: "Creamware STS-series sampler program",
    extensions: &["sts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
