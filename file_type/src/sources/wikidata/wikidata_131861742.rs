use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131861742: FileFormat = FileFormat {
    id: 131_861_742,
    puid: "wikidata/131861742",
    name: "GE Signa ximg file",
    extensions: &["ximg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
