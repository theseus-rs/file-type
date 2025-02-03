use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205948: FileFormat = FileFormat {
    id: 28_205_948,
    source_type: SourceType::Wikidata,
    name: "Dr. Halo Bitmap",
    extensions: &["cut"],
    media_types: &["application/dr-halo"],
    internal_signatures: &[],
    related_formats: &[],
};
