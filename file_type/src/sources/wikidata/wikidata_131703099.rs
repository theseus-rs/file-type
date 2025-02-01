use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131703099: FileFormat = FileFormat {
    id: 131_703_099,
    puid: "wikidata/131703099",
    name: "VERA output file",
    extensions: &["h5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
