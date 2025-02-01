use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272301: FileFormat = FileFormat {
    id: 111_272_301,
    puid: "wikidata/111272301",
    name: "Ensoniq ASR instrument file",
    extensions: &["efa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
