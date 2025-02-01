use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113045074: FileFormat = FileFormat {
    id: 113_045_074,
    puid: "wikidata/113045074",
    name: "PTC G-Plug Granite file",
    extensions: &["g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
