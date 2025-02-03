use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757836: FileFormat = FileFormat {
    id: 28_757_836,
    source_type: SourceType::Wikidata,
    name: "Geany project",
    extensions: &["geany"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
