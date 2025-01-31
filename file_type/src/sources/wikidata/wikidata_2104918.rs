use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2104918: FileFormat = FileFormat {
    id: 2_104_918,
    puid: "wikidata/2104918",
    name: "Portable Sound Format",
    extensions: &["minipsf", "psf", "psflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
