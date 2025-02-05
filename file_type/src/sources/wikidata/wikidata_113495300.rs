use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113495300: FileFormat = FileFormat {
    id: 113_495_300,
    source_type: SourceType::Wikidata,
    name: "JPEG XS File Format",
    extensions: &["jxs"],
    media_types: &["image/jxs"],
    signatures: &[],
    related_formats: &[],
};
