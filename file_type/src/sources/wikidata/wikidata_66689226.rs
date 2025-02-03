use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66689226: FileFormat = FileFormat {
    id: 66_689_226,
    source_type: SourceType::Wikidata,
    name: "Access Add-in Data",
    extensions: &["mdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
