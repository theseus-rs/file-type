use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130240180: FileFormat = FileFormat {
    id: 130_240_180,
    source_type: SourceType::Wikidata,
    name: "Liquid template file",
    extensions: &["liquid"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
