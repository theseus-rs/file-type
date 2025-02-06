use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72274847: FileFormat = FileFormat {
    id: 72_274_847,
    source_type: SourceType::Wikidata,
    name: "Maestro molecular model",
    extensions: &["mae"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
