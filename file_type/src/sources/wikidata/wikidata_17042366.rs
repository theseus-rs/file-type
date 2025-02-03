use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17042366: FileFormat = FileFormat {
    id: 17_042_366,
    source_type: SourceType::Wikidata,
    name: "id Software Music Format",
    extensions: &["imf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
