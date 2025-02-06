use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551372: FileFormat = FileFormat {
    id: 28_551_372,
    source_type: SourceType::Wikidata,
    name: "Adobe Monitor Setup File",
    extensions: &["ams"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
