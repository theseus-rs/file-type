use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130290522: FileFormat = FileFormat {
    id: 130_290_522,
    puid: "wikidata/130290522",
    name: "Meson file format",
    extensions: &["meson.build"],
    media_types: &["text/x-meson"],
    internal_signatures: &[],
    related_formats: &[],
};
