use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205948: FileFormat = FileFormat {
    id: 28_205_948,
    source_type: SourceType::Wikidata,
    name: "Dr. Halo Bitmap",
    extensions: &["cut"],
    media_types: &["application/dr-halo"],
    signatures: &[],
    related_formats: &[],
};
