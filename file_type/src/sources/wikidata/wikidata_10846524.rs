use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10846524: FileFormat = FileFormat {
    id: 10_846_524,
    puid: "wikidata/10846524",
    name: "Blu-ray Disk Movie",
    extensions: &["bdm", "bdmv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
