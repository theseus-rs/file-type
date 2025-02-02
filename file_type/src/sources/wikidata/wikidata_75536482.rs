use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75536482: FileFormat = FileFormat {
    id: 75_536_482,
    source_type: SourceType::Wikidata,
    name: "Ulead Photo Express image",
    extensions: &["upx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
