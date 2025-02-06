use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27492527: FileFormat = FileFormat {
    id: 27_492_527,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 15.00)",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    signatures: &[],
    related_formats: &[],
};
