use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342746: FileFormat = FileFormat {
    id: 111_342_746,
    source_type: SourceType::Wikidata,
    name: "Creamware STS-series sampler program",
    extensions: &["sts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
