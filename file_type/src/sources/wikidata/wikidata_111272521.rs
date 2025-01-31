use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272521: FileFormat = FileFormat {
    id: 111_272_521,
    puid: "wikidata/111272521",
    name: "Ensoniq VFX-SD instrument file",
    extensions: &["efv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
