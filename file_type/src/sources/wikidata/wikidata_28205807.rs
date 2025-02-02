use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205807: FileFormat = FileFormat {
    id: 28_205_807,
    source_type: SourceType::Wikidata,
    name: "Palette Format",
    extensions: &["pal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
