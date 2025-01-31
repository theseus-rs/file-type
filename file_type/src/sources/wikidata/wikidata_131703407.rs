use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131703407: FileFormat = FileFormat {
    id: 131_703_407,
    puid: "wikidata/131703407",
    name: "CONVERGE CFD file format",
    extensions: &["h5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
