use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344736: FileFormat = FileFormat {
    id: 28_344_736,
    source_type: SourceType::Wikidata,
    name: "Macintosh resource file",
    extensions: &["dfont", "rsrc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
