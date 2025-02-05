use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126367768: FileFormat = FileFormat {
    id: 126_367_768,
    source_type: SourceType::Wikidata,
    name: "KPhotoAlbum file",
    extensions: &["kim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
