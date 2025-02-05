use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111356290: FileFormat = FileFormat {
    id: 111_356_290,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif ES sample data file",
    extensions: &["w8v"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
