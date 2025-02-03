use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854992: FileFormat = FileFormat {
    id: 105_854_992,
    source_type: SourceType::Wikidata,
    name: "ArcPad bookmarks",
    extensions: &["apx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
