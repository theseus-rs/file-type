use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27492801: FileFormat = FileFormat {
    id: 27_492_801,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 15.06)",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
