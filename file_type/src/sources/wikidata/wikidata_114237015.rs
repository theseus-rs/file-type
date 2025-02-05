use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114237015: FileFormat = FileFormat {
    id: 114_237_015,
    source_type: SourceType::Wikidata,
    name: "Dialog Script",
    extensions: &["dlg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
