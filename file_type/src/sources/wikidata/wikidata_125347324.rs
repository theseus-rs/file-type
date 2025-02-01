use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125347324: FileFormat = FileFormat {
    id: 125_347_324,
    puid: "wikidata/125347324",
    name: "Binary Grid File",
    extensions: &["grb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
