use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130362694: FileFormat = FileFormat {
    id: 130_362_694,
    source_type: SourceType::Wikidata,
    name: "Myghty file format",
    extensions: &["myt"],
    media_types: &["application/x-myghty"],
    internal_signatures: &[],
    related_formats: &[],
};
