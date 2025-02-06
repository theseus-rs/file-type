use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28018477: FileFormat = FileFormat {
    id: 28_018_477,
    source_type: SourceType::Wikidata,
    name: "Indeo Video Format",
    extensions: &["ivf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
