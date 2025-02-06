use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71274764: FileFormat = FileFormat {
    id: 71_274_764,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Template",
    extensions: &["cdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
