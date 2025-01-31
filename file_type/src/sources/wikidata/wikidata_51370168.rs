use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51370168: FileFormat = FileFormat {
    id: 51_370_168,
    puid: "wikidata/51370168",
    name: "Postscript Support File",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
