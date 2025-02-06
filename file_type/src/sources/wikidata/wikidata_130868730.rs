use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130868730: FileFormat = FileFormat {
    id: 130_868_730,
    source_type: SourceType::Wikidata,
    name: "ShExC file",
    extensions: &["shex"],
    media_types: &["text/shex"],
    signatures: &[],
    related_formats: &[],
};
