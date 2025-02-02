use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66828771: FileFormat = FileFormat {
    id: 66_828_771,
    source_type: SourceType::Wikidata,
    name: "Thumbs.db:encryptable",
    extensions: &["Thumbs.db:encryptable"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
