use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50498413: FileFormat = FileFormat {
    id: 50_498_413,
    source_type: SourceType::Wikidata,
    name: "Draco File Format",
    extensions: &["drc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
