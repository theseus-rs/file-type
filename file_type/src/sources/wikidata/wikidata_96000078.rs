use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96000078: FileFormat = FileFormat {
    id: 96_000_078,
    puid: "wikidata/96000078",
    name: "NOFF 3D geometry format",
    extensions: &["noff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
