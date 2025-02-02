use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113534197: FileFormat = FileFormat {
    id: 113_534_197,
    source_type: SourceType::Wikidata,
    name: "Capture One Settings File",
    extensions: &["cos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
