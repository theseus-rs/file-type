use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29944086: FileFormat = FileFormat {
    id: 29_944_086,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Impress, version 1.0",
    extensions: &["sxi"],
    media_types: &["application/vnd.sun.xml.impress"],
    internal_signatures: &[],
    related_formats: &[],
};
