use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17042366: FileFormat = FileFormat {
    id: 17_042_366,
    source_type: SourceType::Wikidata,
    name: "id Software Music Format",
    extensions: &["imf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
