use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71264369: FileFormat = FileFormat {
    id: 71_264_369,
    puid: "wikidata/71264369",
    name: "Hippel COmpressed SOng module",
    extensions: &["hipc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
