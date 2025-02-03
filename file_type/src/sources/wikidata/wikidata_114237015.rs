use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114237015: FileFormat = FileFormat {
    id: 114_237_015,
    source_type: SourceType::Wikidata,
    name: "Dialog Script",
    extensions: &["dlg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
