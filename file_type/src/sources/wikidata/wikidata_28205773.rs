use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205773: FileFormat = FileFormat {
    id: 28_205_773,
    source_type: SourceType::Wikidata,
    name: "BioRad confocal image",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
