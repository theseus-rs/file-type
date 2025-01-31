use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207539: FileFormat = FileFormat {
    id: 28_207_539,
    puid: "wikidata/28207539",
    name: "Xerox EDMICS-MMR",
    extensions: &["ed", "mmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
