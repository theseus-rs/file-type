use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113495300: FileFormat = FileFormat {
    id: 113_495_300,
    puid: "wikidata/113495300",
    name: "JPEG XS File Format",
    extensions: &["jxs"],
    media_types: &["image/jxs"],
    internal_signatures: &[],
    related_formats: &[],
};
