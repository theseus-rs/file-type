use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979400: FileFormat = FileFormat {
    id: 27_979_400,
    puid: "wikidata/27979400",
    name: "JPX",
    extensions: &["jpf", "jpx"],
    media_types: &["image/jpx", "image/jpx"],
    internal_signatures: &[],
    related_formats: &[],
};
