use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440962: FileFormat = FileFormat {
    id: 111_440_962,
    source_type: SourceType::Wikidata,
    name: "Visual Basic UserControl Object File",
    extensions: &["ctl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
