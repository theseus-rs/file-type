use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113501142: FileFormat = FileFormat {
    id: 113_501_142,
    puid: "wikidata/113501142",
    name: "Cintel Raw Image",
    extensions: &["cri", "dvcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
