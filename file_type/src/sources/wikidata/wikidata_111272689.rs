use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272689: FileFormat = FileFormat {
    id: 111_272_689,
    source_type: SourceType::Wikidata,
    name: "Farandoyle linear module format",
    extensions: &["f2r"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
