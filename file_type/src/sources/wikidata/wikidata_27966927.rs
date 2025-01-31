use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966927: FileFormat = FileFormat {
    id: 27_966_927,
    puid: "wikidata/27966927",
    name: "PSF2",
    extensions: &["minipsf2", "psf2", "psf2lib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
