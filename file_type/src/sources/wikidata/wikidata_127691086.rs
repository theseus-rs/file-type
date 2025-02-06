use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127691086: FileFormat = FileFormat {
    id: 127_691_086,
    source_type: SourceType::Wikidata,
    name: "Dart file",
    extensions: &["dart"],
    media_types: &["text/x-dart"],
    signatures: &[],
    related_formats: &[],
};
