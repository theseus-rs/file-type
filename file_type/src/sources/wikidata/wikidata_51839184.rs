use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51839184: FileFormat = FileFormat {
    id: 51_839_184,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher",
    extensions: &["gen"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
