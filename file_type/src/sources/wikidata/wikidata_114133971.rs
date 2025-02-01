use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114133971: FileFormat = FileFormat {
    id: 114_133_971,
    puid: "wikidata/114133971",
    name: "MSI Molfile",
    extensions: &["msm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
