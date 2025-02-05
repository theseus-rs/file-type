use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551363: FileFormat = FileFormat {
    id: 28_551_363,
    source_type: SourceType::Wikidata,
    name: "Adobe Levels File",
    extensions: &["alv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
