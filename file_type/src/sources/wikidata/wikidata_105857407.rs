use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857407: FileFormat = FileFormat {
    id: 105_857_407,
    source_type: SourceType::Wikidata,
    name: "jalbum image info",
    extensions: &["jpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
