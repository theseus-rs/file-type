use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205983: FileFormat = FileFormat {
    id: 28_205_983,
    source_type: SourceType::Wikidata,
    name: "Radiance Scene Description",
    extensions: &["rad"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
