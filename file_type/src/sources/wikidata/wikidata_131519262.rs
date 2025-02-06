use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131519262: FileFormat = FileFormat {
    id: 131_519_262,
    source_type: SourceType::Wikidata,
    name: "Stimulate Signal Parameters",
    extensions: &["spr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
