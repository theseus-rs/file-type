use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111356268: FileFormat = FileFormat {
    id: 111_356_268,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif ES 'voices' format",
    extensions: &["w7v"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
