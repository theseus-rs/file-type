use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27478595: FileFormat = FileFormat {
    id: 27_478_595,
    source_type: SourceType::Wikidata,
    name: "7z, version 0.2 (with compression methods version 4.27, distributed with 7zip v4.27)",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    signatures: &[],
    related_formats: &[],
};
