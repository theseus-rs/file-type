use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49243748: FileFormat = FileFormat {
    id: 49_243_748,
    source_type: SourceType::Wikidata,
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
