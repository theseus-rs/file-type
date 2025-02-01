use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131161644: FileFormat = FileFormat {
    id: 131_161_644,
    puid: "wikidata/131161644",
    name: "SRCINFO file format",
    extensions: &["SRCINFO"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
