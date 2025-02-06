use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272667: FileFormat = FileFormat {
    id: 111_272_667,
    source_type: SourceType::Wikidata,
    name: "Logic EXS24 instrument file",
    extensions: &["exs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
