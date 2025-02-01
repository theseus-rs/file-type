use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109944419: FileFormat = FileFormat {
    id: 109_944_419,
    puid: "wikidata/109944419",
    name: "BriefCase file format",
    extensions: &["brc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
