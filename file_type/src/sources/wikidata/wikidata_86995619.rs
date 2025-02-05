use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86995619: FileFormat = FileFormat {
    id: 86_995_619,
    source_type: SourceType::Wikidata,
    name: "PaperPort MAX 8-12",
    extensions: &["max"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
