use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_930281: FileFormat = FileFormat {
    id: 930_281,
    source_type: SourceType::Wikidata,
    name: "Windows thumbnail cache",
    extensions: &["db"],
    media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
    signatures: &[],
    related_formats: &[],
};
