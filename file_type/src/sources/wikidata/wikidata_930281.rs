use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_930281: FileFormat = FileFormat {
    id: 930_281,
    source_type: SourceType::Wikidata,
    name: "Windows thumbnail cache",
    extensions: &["db"],
    media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
    internal_signatures: &[],
    related_formats: &[],
};
