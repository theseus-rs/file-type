use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009607: FileFormat = FileFormat {
    id: 111_009_607,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Letterhead File format",
    extensions: &["let"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
