use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551363: FileFormat = FileFormat {
    id: 28_551_363,
    source_type: SourceType::Wikidata,
    name: "Adobe Levels File",
    extensions: &["alv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
