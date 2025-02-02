use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979542: FileFormat = FileFormat {
    id: 27_979_542,
    source_type: SourceType::Wikidata,
    name: "BookmarkData",
    extensions: &["sfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
