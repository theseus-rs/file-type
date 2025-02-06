use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009733: FileFormat = FileFormat {
    id: 111_009_733,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Note Card File format",
    extensions: &["not"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
