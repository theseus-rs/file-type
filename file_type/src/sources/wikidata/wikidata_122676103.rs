use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122676103: FileFormat = FileFormat {
    id: 122_676_103,
    source_type: SourceType::Wikidata,
    name: "JASC Brush File",
    extensions: &["jbr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
