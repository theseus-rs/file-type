use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28345358: FileFormat = FileFormat {
    id: 28_345_358,
    source_type: SourceType::Wikidata,
    name: "Safari bookmarks",
    extensions: &["plist"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
