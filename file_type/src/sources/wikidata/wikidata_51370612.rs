use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51370612: FileFormat = FileFormat {
    id: 51_370_612,
    source_type: SourceType::Wikidata,
    name: "Inkwriter/Notetaker Document",
    extensions: &["pwi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
