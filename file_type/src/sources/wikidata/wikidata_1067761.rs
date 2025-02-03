use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1067761: FileFormat = FileFormat {
    id: 1_067_761,
    source_type: SourceType::Wikidata,
    name: "Windows Media Audio 9 Lossless",
    extensions: &["wma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
