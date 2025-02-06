use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28345358: FileFormat = FileFormat {
    id: 28_345_358,
    source_type: SourceType::Wikidata,
    name: "Safari bookmarks",
    extensions: &["plist"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
