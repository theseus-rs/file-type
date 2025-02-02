use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109944655: FileFormat = FileFormat {
    id: 109_944_655,
    source_type: SourceType::Wikidata,
    name: "Image Systems file format",
    extensions: &["igs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
