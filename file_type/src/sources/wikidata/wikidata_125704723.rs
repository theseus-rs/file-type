use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125704723: FileFormat = FileFormat {
    id: 125_704_723,
    puid: "wikidata/125704723",
    name: "OpenOffice.org 1.0 Master Document",
    extensions: &["sxg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
