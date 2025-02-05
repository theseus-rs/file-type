use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758107: FileFormat = FileFormat {
    id: 28_758_107,
    source_type: SourceType::Wikidata,
    name: "Instant Artist GFX",
    extensions: &["gfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
