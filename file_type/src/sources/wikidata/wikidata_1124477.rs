use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1124477: FileFormat = FileFormat {
    id: 1_124_477,
    source_type: SourceType::Wikidata,
    name: "Efficient XML Interchange",
    extensions: &["exi"],
    media_types: &["application/exi"],
    signatures: &[],
    related_formats: &[],
};
