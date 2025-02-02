use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114061472: FileFormat = FileFormat {
    id: 114_061_472,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Spreadsheet, version 1.3",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    internal_signatures: &[],
    related_formats: &[],
};
