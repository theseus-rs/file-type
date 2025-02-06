use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1067761: FileFormat = FileFormat {
    id: 1_067_761,
    source_type: SourceType::Wikidata,
    name: "Windows Media Audio 9 Lossless",
    extensions: &["wma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
