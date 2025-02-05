use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51913488: FileFormat = FileFormat {
    id: 51_913_488,
    source_type: SourceType::Wikidata,
    name: "Fractal Design Painter RIFF Bitmap Graphics",
    extensions: &["rif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
