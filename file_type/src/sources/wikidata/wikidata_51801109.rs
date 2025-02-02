use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51801109: FileFormat = FileFormat {
    id: 51_801_109,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Toolbar",
    extensions: &["xlb"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
