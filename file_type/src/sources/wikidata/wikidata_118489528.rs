use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118489528: FileFormat = FileFormat {
    id: 118_489_528,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Workspace File 5/95",
    extensions: &["xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
