use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112669255: FileFormat = FileFormat {
    id: 112_669_255,
    source_type: SourceType::Wikidata,
    name: "JSR-184 format",
    extensions: &["m3g"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
