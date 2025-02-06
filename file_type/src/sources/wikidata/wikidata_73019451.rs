use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73019451: FileFormat = FileFormat {
    id: 73_019_451,
    source_type: SourceType::Wikidata,
    name: "Picture Publisher Bitmap, version 5.0",
    extensions: &["pp5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
