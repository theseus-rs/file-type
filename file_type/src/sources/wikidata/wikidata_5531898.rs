use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5531898: FileFormat = FileFormat {
    id: 5_531_898,
    puid: "wikidata/5531898",
    name: "General Exchange Format",
    extensions: &["gxf"],
    media_types: &["application/gxf"],
    internal_signatures: &[],
    related_formats: &[],
};
