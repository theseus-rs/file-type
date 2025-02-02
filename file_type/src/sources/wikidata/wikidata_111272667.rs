use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272667: FileFormat = FileFormat {
    id: 111_272_667,
    source_type: SourceType::Wikidata,
    name: "Logic EXS24 instrument file",
    extensions: &["exs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
