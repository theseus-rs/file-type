use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944334: FileFormat = FileFormat {
    id: 29_944_334,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Impress template, version 1.0",
    extensions: &["sti"],
    media_types: &["application/vnd.sun.xml.impress.template"],
    signatures: &[],
    related_formats: &[],
};
