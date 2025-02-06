use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551383: FileFormat = FileFormat {
    id: 28_551_383,
    source_type: SourceType::Wikidata,
    name: "Adobe Replace Color/Color Range File",
    extensions: &["axt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
