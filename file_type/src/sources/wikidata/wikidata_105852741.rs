use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852741: FileFormat = FileFormat {
    id: 105_852_741,
    puid: "wikidata/105852741",
    name: "Snagit Windows Profile",
    extensions: &["snagprof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
