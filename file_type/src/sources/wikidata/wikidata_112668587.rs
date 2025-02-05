use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112668587: FileFormat = FileFormat {
    id: 112_668_587,
    source_type: SourceType::Wikidata,
    name: "Lightscape Material",
    extensions: &["atr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
