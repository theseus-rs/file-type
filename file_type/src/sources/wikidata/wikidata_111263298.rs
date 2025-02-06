use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263298: FileFormat = FileFormat {
    id: 111_263_298,
    source_type: SourceType::Wikidata,
    name: "Digilink format",
    extensions: &["dig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
