use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51370612: FileFormat = FileFormat {
    id: 51_370_612,
    source_type: SourceType::Wikidata,
    name: "Inkwriter/Notetaker Document",
    extensions: &["pwi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
