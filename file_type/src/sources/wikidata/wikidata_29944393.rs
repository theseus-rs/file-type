use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29944393: FileFormat = FileFormat {
    id: 29_944_393,
    puid: "wikidata/29944393",
    name: "OpenOffice Writer template, version 1.0",
    extensions: &["stw"],
    media_types: &["application/vnd.sun.xml.writer.template"],
    internal_signatures: &[],
    related_formats: &[],
};
