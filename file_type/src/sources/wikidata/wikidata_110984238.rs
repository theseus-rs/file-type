use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110984238: FileFormat = FileFormat {
    id: 110_984_238,
    source_type: SourceType::Wikidata,
    name: "Ulead Image Sequence",
    extensions: &["uis"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
