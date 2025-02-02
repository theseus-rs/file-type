use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126367768: FileFormat = FileFormat {
    id: 126_367_768,
    source_type: SourceType::Wikidata,
    name: "KPhotoAlbum file",
    extensions: &["kim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
