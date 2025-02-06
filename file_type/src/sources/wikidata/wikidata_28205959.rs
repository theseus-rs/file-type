use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205959: FileFormat = FileFormat {
    id: 28_205_959,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Red Channel",
    extensions: &["imr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
