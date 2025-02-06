use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118669865: FileFormat = FileFormat {
    id: 118_669_865,
    source_type: SourceType::Wikidata,
    name: "Manga Studio 3D Dialog file",
    extensions: &["csd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
