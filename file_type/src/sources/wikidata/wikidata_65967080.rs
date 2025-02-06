use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65967080: FileFormat = FileFormat {
    id: 65_967_080,
    source_type: SourceType::Wikidata,
    name: "Sketch file format",
    extensions: &["sketch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
