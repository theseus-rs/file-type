use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127785881: FileFormat = FileFormat {
    id: 127_785_881,
    source_type: SourceType::Wikidata,
    name: "Modula-2 file",
    extensions: &["mod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
