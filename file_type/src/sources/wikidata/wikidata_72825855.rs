use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72825855: FileFormat = FileFormat {
    id: 72_825_855,
    puid: "wikidata/72825855",
    name: "OpenCanvas Image",
    extensions: &["oci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
