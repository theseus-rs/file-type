use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121742883: FileFormat = FileFormat {
    id: 121_742_883,
    source_type: SourceType::Wikidata,
    name: "The Master Genealogist Project",
    extensions: &["pjc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
