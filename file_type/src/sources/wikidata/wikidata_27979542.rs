use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979542: FileFormat = FileFormat {
    id: 27_979_542,
    source_type: SourceType::Wikidata,
    name: "BookmarkData",
    extensions: &["sfl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
