use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28758107: FileFormat = FileFormat {
    id: 28_758_107,
    source_type: SourceType::Wikidata,
    name: "Instant Artist GFX",
    extensions: &["gfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
