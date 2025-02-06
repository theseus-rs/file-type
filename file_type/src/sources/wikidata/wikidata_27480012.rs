use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27480012: FileFormat = FileFormat {
    id: 27_480_012,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 4.45)",
    extensions: &["7z"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
