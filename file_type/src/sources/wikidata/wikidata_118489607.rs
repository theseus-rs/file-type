use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118489607: FileFormat = FileFormat {
    id: 118_489_607,
    puid: "wikidata/118489607",
    name: "Adobe Air 2.5",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
