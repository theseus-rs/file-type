use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99850841: FileFormat = FileFormat {
    id: 99_850_841,
    puid: "wikidata/99850841",
    name: "Picture Publisher Bitmap 6-10",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
