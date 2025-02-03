use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27479005: FileFormat = FileFormat {
    id: 27_479_005,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 4.27, distributed with 7zip v4.28)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
