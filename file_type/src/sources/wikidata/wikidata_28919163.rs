use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919163: FileFormat = FileFormat {
    id: 28_919_163,
    puid: "wikidata/28919163",
    name: "Cult3D",
    extensions: &["cd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
