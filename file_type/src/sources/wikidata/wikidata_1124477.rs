use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1124477: FileFormat = FileFormat {
    id: 1_124_477,
    source_type: SourceType::Wikidata,
    name: "Efficient XML Interchange",
    extensions: &["exi"],
    media_types: &["application/exi"],
    internal_signatures: &[],
    related_formats: &[],
};
