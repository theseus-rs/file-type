use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113494624: FileFormat = FileFormat {
    id: 113_494_624,
    source_type: SourceType::Wikidata,
    name: "Persuasion Presentation Interchange File",
    extensions: &["prf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
