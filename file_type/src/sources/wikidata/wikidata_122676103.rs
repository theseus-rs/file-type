use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122676103: FileFormat = FileFormat {
    id: 122_676_103,
    source_type: SourceType::Wikidata,
    name: "JASC Brush File",
    extensions: &["jbr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
