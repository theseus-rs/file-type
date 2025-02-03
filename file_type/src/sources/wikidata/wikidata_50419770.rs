use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50419770: FileFormat = FileFormat {
    id: 50_419_770,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Graphics Metafile",
    extensions: &["wpg"],
    media_types: &["image/x-wpg"],
    internal_signatures: &[],
    related_formats: &[],
};
