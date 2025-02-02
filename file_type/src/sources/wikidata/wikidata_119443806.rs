use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119443806: FileFormat = FileFormat {
    id: 119_443_806,
    source_type: SourceType::Wikidata,
    name: "Map Template File",
    extensions: &["axt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
