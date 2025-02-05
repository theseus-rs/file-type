use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959801: FileFormat = FileFormat {
    id: 27_959_801,
    source_type: SourceType::Wikidata,
    name: "Ableton Groove File",
    extensions: &["agr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
