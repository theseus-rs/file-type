use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116923685: FileFormat = FileFormat {
    id: 116_923_685,
    source_type: SourceType::Wikidata,
    name: "Super Duper Music Looper File",
    extensions: &["sdml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
