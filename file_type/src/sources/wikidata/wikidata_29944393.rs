use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944393: FileFormat = FileFormat {
    id: 29_944_393,
    source_type: SourceType::Wikidata,
    name: "OpenOffice Writer template, version 1.0",
    extensions: &["stw"],
    media_types: &["application/vnd.sun.xml.writer.template"],
    signatures: &[],
    related_formats: &[],
};
