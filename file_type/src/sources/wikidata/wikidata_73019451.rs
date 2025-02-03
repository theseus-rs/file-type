use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73019451: FileFormat = FileFormat {
    id: 73_019_451,
    source_type: SourceType::Wikidata,
    name: "Picture Publisher Bitmap, version 5.0",
    extensions: &["pp5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
