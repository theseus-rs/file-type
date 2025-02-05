use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904450: FileFormat = FileFormat {
    id: 29_904_450,
    source_type: SourceType::Wikidata,
    name: "Presentation Manager Metafile",
    extensions: &["met"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
