use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111332609: FileFormat = FileFormat {
    id: 111_332_609,
    puid: "wikidata/111332609",
    name: "Orion Sampler program",
    extensions: &["osp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
