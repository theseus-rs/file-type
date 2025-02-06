use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551228: FileFormat = FileFormat {
    id: 28_551_228,
    source_type: SourceType::Wikidata,
    name: "Adobe Action File",
    extensions: &["atn"],
    media_types: &["application/x-photoshop"],
    signatures: &[],
    related_formats: &[],
};
