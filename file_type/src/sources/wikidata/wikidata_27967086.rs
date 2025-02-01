use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967086: FileFormat = FileFormat {
    id: 27_967_086,
    puid: "wikidata/27967086",
    name: "Soundfactory",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
