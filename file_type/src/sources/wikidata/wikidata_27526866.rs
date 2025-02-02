use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27526866: FileFormat = FileFormat {
    id: 27_526_866,
    source_type: SourceType::Wikidata,
    name: "Write for Windows Document, version 3.1",
    extensions: &["wri"],
    media_types: &["application/x-mswrite"],
    internal_signatures: &[],
    related_formats: &[],
};
