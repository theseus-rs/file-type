use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342746: FileFormat = FileFormat {
    id: 111_342_746,
    puid: "wikidata/111342746",
    name: "Creamware STS-series sampler program",
    extensions: &["sts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
