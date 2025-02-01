use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118610691: FileFormat = FileFormat {
    id: 118_610_691,
    puid: "wikidata/118610691",
    name: "Visual C++ IntelliSense Database file",
    extensions: &["ncb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
