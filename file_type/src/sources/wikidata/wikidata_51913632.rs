use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51913632: FileFormat = FileFormat {
    id: 51_913_632,
    source_type: SourceType::Wikidata,
    name: "SDSC Image Tool Run-Length Encoded Bitmap",
    extensions: &["rle"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
