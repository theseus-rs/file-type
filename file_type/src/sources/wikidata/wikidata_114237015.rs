use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114237015: FileFormat = FileFormat {
    id: 114_237_015,
    puid: "wikidata/114237015",
    name: "Dialog Script",
    extensions: &["dlg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
