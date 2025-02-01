use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_930281: FileFormat = FileFormat {
    id: 930_281,
    puid: "wikidata/930281",
    name: "Windows thumbnail cache",
    extensions: &["db"],
    media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
    internal_signatures: &[],
    related_formats: &[],
};
