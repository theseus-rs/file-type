use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440962: FileFormat = FileFormat {
    id: 111_440_962,
    source_type: SourceType::Wikidata,
    name: "Visual Basic UserControl Object File",
    extensions: &["ctl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
