use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272524: FileFormat = FileFormat {
    id: 111_272_524,
    puid: "wikidata/111272524",
    name: "Ensoniq instrument file",
    extensions: &["efx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
