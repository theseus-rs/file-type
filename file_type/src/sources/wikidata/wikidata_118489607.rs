use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118489607: FileFormat = FileFormat {
    id: 118_489_607,
    source_type: SourceType::Wikidata,
    name: "Adobe Air 2.5",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    signatures: &[],
    related_formats: &[],
};
