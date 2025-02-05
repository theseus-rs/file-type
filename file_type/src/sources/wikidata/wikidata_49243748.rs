use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49243748: FileFormat = FileFormat {
    id: 49_243_748,
    source_type: SourceType::Wikidata,
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
