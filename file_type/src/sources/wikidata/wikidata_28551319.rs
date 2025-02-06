use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551319: FileFormat = FileFormat {
    id: 28_551_319,
    source_type: SourceType::Wikidata,
    name: "Adobe Custom Kernel File",
    extensions: &["acf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
