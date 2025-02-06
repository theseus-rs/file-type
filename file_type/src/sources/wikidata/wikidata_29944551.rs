use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944551: FileFormat = FileFormat {
    id: 29_944_551,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Calc, version 1.0",
    extensions: &["sxc"],
    media_types: &["application/vnd.sun.xml.calc"],
    signatures: &[],
    related_formats: &[],
};
