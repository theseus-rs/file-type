use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27526866: FileFormat = FileFormat {
    id: 27_526_866,
    source_type: SourceType::Wikidata,
    name: "Write for Windows Document, version 3.1",
    extensions: &["wri"],
    media_types: &["application/x-mswrite"],
    signatures: &[],
    related_formats: &[],
};
