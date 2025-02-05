use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27480238: FileFormat = FileFormat {
    id: 27_480_238,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 4.61)",
    extensions: &["7z"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
