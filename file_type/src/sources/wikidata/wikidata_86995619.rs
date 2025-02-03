use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86995619: FileFormat = FileFormat {
    id: 86_995_619,
    source_type: SourceType::Wikidata,
    name: "PaperPort MAX 8-12",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
