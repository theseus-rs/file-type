use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849587: FileFormat = FileFormat {
    id: 105_849_587,
    source_type: SourceType::Wikidata,
    name: "Cube LUT format (with rem)",
    extensions: &["cube"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
