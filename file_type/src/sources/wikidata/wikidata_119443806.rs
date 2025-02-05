use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119443806: FileFormat = FileFormat {
    id: 119_443_806,
    source_type: SourceType::Wikidata,
    name: "Map Template File",
    extensions: &["axt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
