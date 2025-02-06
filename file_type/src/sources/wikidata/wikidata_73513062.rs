use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73513062: FileFormat = FileFormat {
    id: 73_513_062,
    source_type: SourceType::Wikidata,
    name: "Pathetic Writer document",
    extensions: &["pw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
