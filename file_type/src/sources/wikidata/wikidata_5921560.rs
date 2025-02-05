use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5921560: FileFormat = FileFormat {
    id: 5_921_560,
    source_type: SourceType::Wikidata,
    name: "Synthetic music mobile application format",
    extensions: &["m3f", "mmf", "mqf"],
    media_types: &["application/vnd.smaf"],
    signatures: &[],
    related_formats: &[],
};
