use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967211: FileFormat = FileFormat {
    id: 27_967_211,
    source_type: SourceType::Wikidata,
    name: "Pumatracker module",
    extensions: &["puma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
