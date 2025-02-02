use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28786544: FileFormat = FileFormat {
    id: 28_786_544,
    source_type: SourceType::Wikidata,
    name: "Netscape bookmarks",
    extensions: &["htm", "html"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
