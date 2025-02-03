use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121742883: FileFormat = FileFormat {
    id: 121_742_883,
    source_type: SourceType::Wikidata,
    name: "The Master Genealogist Project",
    extensions: &["pjc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
