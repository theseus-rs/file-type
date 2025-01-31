use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012501: FileFormat = FileFormat {
    id: 47_012_501,
    puid: "wikidata/47012501",
    name: "OmniPage Pro Document file format",
    extensions: &["met", "opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
