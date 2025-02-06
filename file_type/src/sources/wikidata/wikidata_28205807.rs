use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205807: FileFormat = FileFormat {
    id: 28_205_807,
    source_type: SourceType::Wikidata,
    name: "Palette Format",
    extensions: &["pal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
