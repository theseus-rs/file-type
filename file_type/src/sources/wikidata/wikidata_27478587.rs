use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27478587: FileFormat = FileFormat {
    id: 27_478_587,
    source_type: SourceType::Wikidata,
    name:
        "7z, version 0.2 (with compression methods version 4.16 beta, distributed with 7zip v4.26)",
    extensions: &["7z"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
