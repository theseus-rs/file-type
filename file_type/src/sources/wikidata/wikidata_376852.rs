use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_376852: FileFormat = FileFormat {
    id: 376_852,
    source_type: SourceType::Wikidata,
    name: "Extended Module",
    extensions: &["xm"],
    media_types: &["audio/xm"],
    signatures: &[],
    related_formats: &[],
};
