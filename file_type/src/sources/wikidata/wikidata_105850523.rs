use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850523: FileFormat = FileFormat {
    id: 105_850_523,
    source_type: SourceType::Wikidata,
    name: "Camtasia Studio Project",
    extensions: &["camproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
