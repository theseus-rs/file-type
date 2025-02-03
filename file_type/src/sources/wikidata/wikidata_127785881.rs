use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127785881: FileFormat = FileFormat {
    id: 127_785_881,
    source_type: SourceType::Wikidata,
    name: "Modula-2 file",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
