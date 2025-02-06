use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51801521: FileFormat = FileFormat {
    id: 51_801_521,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Workspace",
    extensions: &["xlw"],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[],
    related_formats: &[],
};
