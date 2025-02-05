use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944655: FileFormat = FileFormat {
    id: 109_944_655,
    source_type: SourceType::Wikidata,
    name: "Image Systems file format",
    extensions: &["igs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
