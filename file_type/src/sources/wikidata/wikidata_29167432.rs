use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167432: FileFormat = FileFormat {
    id: 29_167_432,
    source_type: SourceType::Wikidata,
    name: "NuFX",
    extensions: &["bxy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
