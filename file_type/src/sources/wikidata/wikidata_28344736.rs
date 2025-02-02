use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344736: FileFormat = FileFormat {
    id: 28_344_736,
    source_type: SourceType::Wikidata,
    name: "Macintosh resource file",
    extensions: &["dfont", "rsrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
