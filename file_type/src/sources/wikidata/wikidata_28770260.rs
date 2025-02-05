use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770260: FileFormat = FileFormat {
    id: 28_770_260,
    source_type: SourceType::Wikidata,
    name: "Hugo",
    extensions: &["hex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
