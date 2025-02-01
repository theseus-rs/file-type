use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119443806: FileFormat = FileFormat {
    id: 119_443_806,
    puid: "wikidata/119443806",
    name: "Map Template File",
    extensions: &["axt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
