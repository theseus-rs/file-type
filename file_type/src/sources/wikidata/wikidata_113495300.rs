use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113495300: FileFormat = FileFormat {
    id: 113_495_300,
    source_type: SourceType::Wikidata,
    name: "JPEG XS File Format",
    extensions: &["jxs"],
    media_types: &["image/jxs"],
    internal_signatures: &[],
    related_formats: &[],
};
