use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975912: FileFormat = FileFormat {
    id: 28_975_912,
    puid: "wikidata/28975912",
    name: "XGL",
    extensions: &["xgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
