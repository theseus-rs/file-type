use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009753: FileFormat = FileFormat {
    id: 111_009_753,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Web Page File format",
    extensions: &["web"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
